
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(convert)]

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
mod snapshot;
#[macro_use]
mod ship;
mod demo;
pub mod sim;
mod tagtree;
