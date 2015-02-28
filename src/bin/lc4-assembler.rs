#![feature(box_syntax, core, env)]

extern crate core;
extern crate lc4;

use std::env::args;

use lc4::assembler::*;

pub fn main() -> () {
  let ref source_file = match args().nth(1) {
    Some(arg) => arg,
    None => {
      println!("Missing source file argument");
      return
    }
  };
  
  let assm_lines: Vec<Assm> = match read_assembly_file(source_file) {
    Err(err) => panic!("{:?}",err),
    Ok(assms) => assms
  };

  let assm_data = assemble(assm_lines);

  println!("Debug labels:");
  for (l,addr) in assm_data.labels.iter() {
    println!("  Label {:?} = {:?}", l, addr);
  }

  println!("");
  for addr in 0..assm_data.heap {
    println!("{:?} {:?}", addr, assm_data.memory[addr as usize]);
  }

  println!("Heap starts at {:?}", assm_data.heap);

}