
use na::{Vec2};
use ncollide::shape::{Cone, Cuboid};
use nphysics::math::{Matrix, Point};
use nphysics::object::{RigidBody};

use constants::{VESSEL_DENSITY, FUELTANK_DENSITY, ENGINE_DENSITY};
use tagtree;

enum Part {
  Vessel { width: f64, length: f64 },
  FuelTank { width: f64, length: f64 },
  Engine { radius: f64, length: f64, group: i32 }
}

impl Part {

    /*
    fn mass (&self) -> f64 {
        match self {
            &Part::Vessel {width, length} => 
                VESSEL_DENSITY * width * length,
            &Part::FuelTank {width, length} =>
                FUELTANK_DENSITY * width * length,
            &Part::Engine {radius, length, group} =>
                ENGINE_DENSITY * radius * length * 0.5
        }
    }
    */

    fn object (&self) -> RigidBody {
        match self {
            &Part::Vessel {width, length} => {
                let geom = Cuboid::new(Vec2::new(width, length));
                RigidBody::new_dynamic(geom, VESSEL_DENSITY, 1.0, 1.0)
            }
            &Part::FuelTank {width, length} => {
                let geom = Cuboid::new(Vec2::new(width, length));
                RigidBody::new_dynamic(geom, FUELTANK_DENSITY, 1.0, 1.0)
            }
            &Part::Engine {radius, length, group} => {
                let geom = Cone::new(length, radius);
                RigidBody::new_dynamic(geom, ENGINE_DENSITY, 1.0, 1.0)
            }
        }
    }
}

struct PartObjectCache {
    part: Part,
    object: RigidBody,
    mass: f64
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
