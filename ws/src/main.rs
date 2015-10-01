#![feature(mpsc_select)]

/// Simple WebSocket server with error handling. It is not necessary to setup logging, but doing
/// so will allow you to see more details about the connection by using the RUST_LOG env variable.

extern crate ecs;
extern crate env_logger;
extern crate rand;
extern crate sim;
extern crate ws;

// Possibly replace with an mio type of channel?
use ecs::{Entity};
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender as ThreadOut};
use std::thread;
use ws::{listen, CloseCode, Handler, Handshake, Message, Sender, Result};

enum Registration {
    Register { client: i32, entity_id_in: ThreadOut<Entity>, out: Sender },
    Unregister { client: i32 }
}

struct Action {
    entity: Entity,
    buf: Vec<u8>
}

struct Snapshot {
    buf: Vec<u8>
}

struct ActionHandler {
    client: i32,
    entity_id: Option<Entity>,
    out: Sender,
    registrations: ThreadOut<Registration>,
    actions: ThreadOut<Action>,
}

impl Handler for ActionHandler {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Opened client connection {}", self.client);
        let (entity_id_in, entity_id_out) = channel();
        let registration = Registration::Register {
            client: self.client,
            entity_id_in: entity_id_in,
            out: self.out.clone()
        };
        self.registrations.send(registration).unwrap();
        self.entity_id = Some(entity_id_out.recv().unwrap());
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Message::Binary(buf) = msg {
            println!("Receive binary message from client {} len {}", self.client, buf.len());
            let action = Action {
                entity: self.entity_id.unwrap(),
                buf: buf
            };
            self.actions.send(action).unwrap();
        }
        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        let unregistration = Registration::Unregister {
            client: self.client,
        };
        self.registrations.send(unregistration).unwrap();
    }

}

fn main () {

    let mut sim = sim::sim::Sim::new();

    let (actions_in, actions_out) = channel();
    let (registrations_in, registrations_out) = channel();
    let (ticks_in, ticks_out) = channel::<()>();

    // Setup logging
    env_logger::init().unwrap();

    let server = thread::Builder::new().name("server".to_string()).spawn(move || {

        // Listen on an address and call the closure for each connection
        listen("0.0.0.0:8081", |out| { 
            let client: i32 = rand::random();
            ActionHandler {
                client: client,
                entity_id: None,
                out: out,
                registrations: registrations_in.clone(),
                actions: actions_in.clone(),
            } 
        }).unwrap()

    }).unwrap();

    let timer = thread::Builder::new().name("timer".to_string()).spawn(move || {

        loop {
            ticks_in.send(()).unwrap();
            thread::sleep_ms(50); // TODO magic number
        }

    }).unwrap();

    let mut clients = HashMap::new();

    loop {

        select! {
            msg = registrations_out.recv() => {
                let registration = msg.unwrap();
                match registration {
                    Registration::Register{client, entity_id_in, out} => {
                        println!("Registering {}", client);
                        if let Some(_) = clients.insert(client, out.clone()) {
                            panic!("Collision registering client {}", client);
                        }
                        let (entity_id, focus_update_buf) = sim.connect(client);
                        entity_id_in.send(entity_id).unwrap();
                        out.send(Message::Binary(focus_update_buf));
                    },
                    Registration::Unregister{client} => {
                        println!("Unregistering {}", client);
                        if let None = clients.remove(&client) {
                            panic!("Tried to remove non-existent client {}", client);
                        }
                    }
                }
            },
            msg = actions_out.recv() => {
                let action = msg.unwrap();
                let entity = action.entity;
                println!("Actioning from {}", entity.id());
                sim.apply_buf(entity, action.buf);
            },
            msg = ticks_out.recv() => {
                // don't care about the value (TODO delta_time)
                println!("Got tick, updating and snapshotting");
                let buf = sim.snapshot_buf();
                for (client, out) in clients.iter() {
                    let send_buf = buf.clone();
                    out.send(Message::Binary(send_buf));
                }
            }
        }

    }

}
