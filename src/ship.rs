
use tagtree;

enum Part {
  Vessel { width: f64, length: f64 },
  FuelTank { width: f64, length: f64 },
  Engine { width: f64, length: f64, group: i32 }
}

struct Beam {
    length: f64
}

struct Attach {
    location: f64,
    rotation: f64
}

type Structure = tagtree::TagTree<Part,Beam,Attach>;

impl Structure {
    
}

fn part(attrs: Part) -> Structure {
    tagtree::TagTree::Leaf(attrs)
}

fn beam(length: f64, parts: Vec<(Attach, Box<Structure>)>) -> Structure {
    tagtree::TagTree::Node(Beam {length: length}, parts)
}

#[test]
fn simple_parts () {
    part(Part::Vessel { width: 2.0, length: 4.0 });
    part(Part::FuelTank { width: 1.0, length: 5.0});
    part(Part::Engine { width: 1.0, length: 2.0, group: 3});
    beam(5.0, Vec::new());
}

#[test]
fn compount_ships () {
}
