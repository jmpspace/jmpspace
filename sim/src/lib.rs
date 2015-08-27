
#![feature(box_patterns)]
#![feature(box_syntax)]

#![allow(dead_code)]

extern crate nalgebra as na;
extern crate ncollide;
extern crate nphysics;
extern crate num;

mod constants;
mod ship;
mod tagtree;

#[no_mangle]
pub extern "C" fn hello_sim() {
  println!("Hello, Rust Sim!");
}
