
extern crate protobuf;

mod contracts;

use contracts::space_server::session::{AuthCredential, AuthRequest};

use protobuf::core::Message;
use protobuf::stream::{CodedOutputStream};

use std::net::TcpStream;
use std::string::String;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    //let mut writer = CodedOutputStream::new(&mut stream);
    println!("Prepping object");
    let mut credential = AuthCredential::new();
    credential.set_username(String::from("John"));
    credential.set_password(String::from("xyz"));
    let mut request = AuthRequest::new();
    request.set_credential(credential);
    //sleep(Duration::new(5,0));
    println!("Writing message");
    println!("Request size is... {}",request.compute_size());
    request.write_length_delimited_to_writer(&mut stream).unwrap();
    //credential.write_length_delimited_to_writer(&mut stream).unwrap();
    //request.write_to(&mut writer).unwrap();
}
