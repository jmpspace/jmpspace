
extern crate byteorder;
extern crate protobuf;

mod contracts;

use byteorder::{BigEndian, WriteBytesExt};

use contracts::space_server::session::{AuthCredential, AuthRequest};

use protobuf::core::Message;
use protobuf::stream::{CodedOutputStream};

use std::io::Write;
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
    //sleep(Duration::new(5,0));
    println!("Writing message");
    let size = request.compute_size() as i32;
    println!("Request size is... {}",size);
    //stream.write(&[0xff]);
    stream.write_i32::<BigEndian>(size);
    let msgVec: Vec<u8> = request.write_to_bytes().unwrap();
    stream.write(msgVec.as_slice());
    //let mut writer = CodedOutputStream::new(&mut stream);
    //writer.write_message(1, &request);
    //writer.write_raw_little_endian32(size).unwrap();
    //request.write_to_writer(&mut stream).unwrap();
    //credential.write_length_delimited_to_writer(&mut stream).unwrap();
    //request.write_to(&mut writer).unwrap();
    //writer.write_message
}
