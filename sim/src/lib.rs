
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

