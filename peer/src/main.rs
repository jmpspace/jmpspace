
extern crate protobuf;

mod contracts;

use contracts::space_server::session::{AuthCredential, AuthRequest};

use protobuf::core::Message;

use std::io::prelude::*;
use std::net::TcpStream;
use std::string::String;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    println!("Prepping object");
    let mut credential = AuthCredential::new();
    credential.set_username(String::from("John"));
    credential.set_password(String::from("xyz"));
    let mut request = AuthRequest::new();
    request.set_credential(credential);
    sleep(Duration::new(5,0));
    println!("Writing message");
    request.write_to_writer(&mut stream);
}
