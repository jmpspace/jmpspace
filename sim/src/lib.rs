
#![feature(box_patterns)]
#![feature(box_syntax)]

#![allow(dead_code)]

extern crate contracts;
extern crate nalgebra as na;
extern crate ncollide;
extern crate nphysics;
extern crate num;

mod constants;
mod ship;
mod tagtree;
mod sim;

use contracts::actions::Action;
use nphysics::world::World;

#[no_mangle]
pub extern "C" fn hello_sim() {
    println!("Hello, Rust Sim!");
}

pub extern "C" fn build_world(a: i32) -> *mut sim::Sim {
    let mut world = World::new();
    let mut sim = Box::new(sim::Sim { a: a, world: world });
    &mut *sim
}

pub extern "C" fn apply_command(world: *mut World, action: Action) {
    // TODO deref world.a
    println!("Apply");
}

pub extern "C" fn world_snapshow(world: *mut World) {
    // TODO deref world.a
    println!("Snapshot");
}
