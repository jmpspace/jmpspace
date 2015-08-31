
#![feature(box_patterns)]
#![feature(box_syntax)]

#![allow(dead_code)]

extern crate nalgebra as na;
extern crate ncollide;
extern crate nphysics;
extern crate num;

//extern crate contracts;

mod constants;
mod ship;
mod tagtree;
mod sim;

//use contracts::actions::Action;
use nphysics::world::World;

#[no_mangle]
pub extern "C" fn hello_sim() {
    println!("Hello, Rust Sim!");
}

#[no_mangle]
pub extern "C" fn build_world(a: i32) -> *mut sim::Sim {
    let world = World::new();
    let mut sim = Box::new(sim::Sim { a: a, world: world });
    &mut *sim
}

#[no_mangle]
pub extern "C" fn apply_command(sim: *mut sim::Sim, a: i32) {
    // TODO deref world.a
    println!("Apply");
}

#[no_mangle]
pub extern "C" fn world_snapshot(sim: *mut sim::Sim) {
    let a = unsafe { (*sim).a };
    println!("Snapshot {}", a);
}
