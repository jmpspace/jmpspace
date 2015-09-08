
#![feature(box_patterns)]
#![feature(box_syntax)]

#![allow(dead_code)]

#[macro_use]
extern crate ecs;
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

#[no_mangle]
pub extern "C" fn hello_sim() {
    println!("Hello, Rust Sim!");
}

#[no_mangle]
pub extern "C" fn build_world(a: i32) -> *mut sim::Sim {
    let mut sim = Box::new(sim::Sim::new(a));
    &mut *sim
}

#[no_mangle]
pub extern "C" fn apply_action(sim: *mut sim::Sim, a: i32) {
    unsafe { (*sim).a = a }
    println!("Apply");
}

#[no_mangle]
pub extern "C" fn update_world(sim: *mut sim::Sim) {
    unsafe { (*sim).update() }
    println!("Update");
}

#[no_mangle]
pub extern "C" fn snapshot_world(sim: *mut sim::Sim) {
    let a = unsafe { (*sim).a };
    println!("Snapshot {}", a);
}
