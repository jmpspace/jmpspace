#![feature(mpsc_select)]

/// Simple WebSocket server with error handling. It is not necessary to setup logging, but doing
/// so will allow you to see more details about the connection by using the RUST_LOG env variable.

extern crate env_logger;
extern crate rand;
extern crate ws;

// Possibly replace with an mio type of channel?
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender as ThreadOut};
use std::thread;
use ws::{listen, CloseCode, Handler, Handshake, Message, Sender, Result};

struct Registration {
    client: i32,
    register: bool,
    out: Option<Sender>
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
        let registration = Registration {
            client: self.client,
            register: true,
            out: Some(self.out.clone())
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
        let unregistration = Registration {
            client: self.client,
            register: false,
            out: None
        };
        self.registrations.send(unregistration).unwrap();
    }

}

fn main () {

    let (registrations_in, registrations_out) = channel();
    let (actions_in, actions_out) = channel();

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

    let mut clients = HashMap::new();

    loop {

        select! {
            msg = registrations_out.recv() => {
                let registration = msg.unwrap();
                let client = registration.client;
                if registration.register {
                    println!("Registering {}", client);
                    if let Some(_) = clients.insert(client, registration) {
                        panic!("Collision registering client {}", client);
                    }
                } else {
                    println!("Unregistering {}", client);
                    if let None = clients.remove(&client) {
                        panic!("Tried to remove non-existent client {}", client);
                    }
                }
            },
            msg = actions_out.recv() => {
                let action = msg.unwrap();
                let client = action.client;
                println!("Actioning from {}", client);
            }
        }

    }



}
