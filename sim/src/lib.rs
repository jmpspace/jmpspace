
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(convert)]

#![feature(ptr_as_ref)]

#![allow(dead_code)]

#[macro_use]
extern crate ecs;
extern crate libc;
extern crate nalgebra as na;
extern crate ncollide;
extern crate nphysics;
extern crate num;
extern crate protobuf;

// JmpSpace dependencies
extern crate contracts;

// This crate
mod constants;
mod demo;
mod physics;
mod ship;
pub mod sim;
mod tagtree;

use libc::{uint8_t, size_t};
use protobuf::core::Message;
use std::ptr;
use std::slice;

#[repr(C)]
pub struct Buffer {
    length: size_t,
    buf: *const uint8_t
}

#[no_mangle]
pub extern "C" fn hello_sim() {
    println!("Hello, Rust Sim!");
}

#[no_mangle]
pub extern "C" fn build_world() -> *mut sim::Sim {
    let mut sim = Box::new(sim::Sim::new());
    &mut *sim
}

#[no_mangle]
pub extern "C" fn connect_client(sim: *mut sim::Sim, client: i32) -> i32 {
    println!("Connect {:?} {}", sim, client); // TODO
    let _ = unsafe { (*sim).connect(client) };
    0
}

#[no_mangle]
pub extern "C" fn update_world(sim: *mut sim::Sim) -> i32 {
    println!("Update {:?}", sim); // TODO
    0
}

#[no_mangle]
pub extern "C" fn apply_action(sim: *mut sim::Sim, 
                               client: i32,
                               buffer: Buffer) -> i32 {
    let length = buffer.length as usize;
    let ref mut sim = unsafe { 
        Option::expect(sim.as_mut(), "Dereference sim")
    };
    let action_slice = unsafe { 
        slice::from_raw_parts(buffer.buf, length)
    };
    let mut action = contracts::actions::Action::new();
    if let Err(_) = action.merge_from_bytes(action_slice) {
        // TODO logging
        // TODO meaningful error codes in header file
        return 1;
    }
    sim.apply(client, &action);
    0
}

#[no_mangle]
pub extern "C" fn snapshot_world(sim: *mut sim::Sim) -> Buffer {
    println!("Snapshot {:?}", sim); // TODO
    let snapshot = unsafe { (*sim).snapshot() };
    let mut snapshot_vec = box Vec::new();
    if let Err(_) = snapshot.write_to_vec(&mut snapshot_vec) {
        // TODO logging
        // TODO meaningful error code
        return Buffer {
            length: 0,
            buf: ptr::null()
        }
    }
    let length = snapshot_vec.len() as size_t;
    println!("Snapshot length {}", length);
    if length == 0 {
        return Buffer {
            length: 0,
            buf: ptr::null()
        }
    }
    let buf = snapshot_vec.as_mut_slice();
    Buffer {
        length: length,
        buf: &buf[0]
    }
}

#[test]
fn simple_ffi() {
    let sim = build_world();
    connect_client(sim, 121642);
}
