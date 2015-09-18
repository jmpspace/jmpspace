#![feature(mpsc_select)]

/// Simple WebSocket server with error handling. It is not necessary to setup logging, but doing
/// so will allow you to see more details about the connection by using the RUST_LOG env variable.

extern crate env_logger;
extern crate rand;
extern crate sim;
extern crate ws;

// Possibly replace with an mio type of channel?
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender as ThreadOut};
use std::thread;
use ws::{listen, CloseCode, Handler, Handshake, Message, Sender, Result};

enum Registration {
    Register { client: i32, out: Sender },
    Unregister { client: i32 }
}

struct Action {
    client: i32,
    buf: Vec<u8>
}

struct Snapshot {
    buf: Vec<u8>
}

struct ActionHandler {
    client: i32,
    out: Sender,
    registrations: ThreadOut<Registration>,
    actions: ThreadOut<Action>,
}

impl Handler for ActionHandler {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Opened client connection {}", self.client);
        let registration = Registration::Register {
            client: self.client,
            out: self.out.clone()
        };
        self.registrations.send(registration).unwrap();
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Message::Binary(buf) = msg {
            println!("Receive binary message from client {} len {}", self.client, buf.len());
            let action = Action {
                client: self.client,
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
                out: out,
                registrations: registrations_in.clone(),
                actions: actions_in.clone(),
            } 
        }).unwrap()

    }).unwrap();

    let timer = thread::Builder::new().name("timer".to_string()).spawn(move || {

        loop {
            ticks_in.send(());
            thread::sleep_ms(50); // TODO magic number
        }

    }).unwrap();

    let mut clients = HashMap::new();

    loop {

        select! {
            msg = registrations_out.recv() => {
                let registration = msg.unwrap();
                match registration {
                    Registration::Register{client, out} => {
                        println!("Registering {}", client);
                        if let Some(_) = clients.insert(client, out) {
                            panic!("Collision registering client {}", client);
                        }
                    sim.connect(client);
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
                let client = action.client;
                println!("Actioning from {}", client);
                sim.apply_buf(client, action.buf);
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
