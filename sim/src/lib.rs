
#![feature(box_patterns)]
#![feature(box_syntax)]

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
mod physics;
mod ship;
mod sim;
mod tagtree;

use libc::{uint8_t, size_t};
use protobuf::core::Message;
use std::slice;

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
pub extern "C" fn apply_action(sim: *mut sim::Sim, 
                               buf: *const uint8_t, 
                               len: size_t) -> i32 {
    let length = len as usize;
    let ref mut sim = unsafe { 
        Option::expect(sim.as_mut(), "Dereference sim")
    };
    let action_slice = unsafe { 
        slice::from_raw_parts(buf, length)
    };
    let mut action = contracts::actions::Action::new();
    if let Err(_) = action.merge_from_bytes(action_slice) {
        // TODO logging
        // TODO meaningful error codes in header file
        return 1;
    }
    sim.apply(&action);
    return 0;
}

#[no_mangle]
pub extern "C" fn update_world(sim: *mut sim::Sim) {
    unsafe { (*sim).update() }
    println!("Update");
}

#[no_mangle]
pub extern "C" fn snapshot_world(sim: *mut sim::Sim) {
    println!("Snapshot {:?}", sim);
}
