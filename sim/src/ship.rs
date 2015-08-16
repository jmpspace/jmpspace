
use na::{Vec2};
use ncollide::shape::{Cone, Cuboid, Cylinder};
use nphysics::object::{RigidBody};

use constants::*;
use tagtree;

enum Part {
    Vessel { width: f64, length: f64 },
    FuelTank { radius: f64, length: f64 },
    Engine { radius: f64, length: f64, group: i32 }
}

struct PointMass {
    center: Vec2<f64>,
    mass: f64
}

impl Part {

    fn mass (&self) -> f64 {
        match self {
            &Part::Vessel {width, length} => {
                VESSEL_DENSITY * width * length
            }
            &Part::FuelTank {radius, length} => {
                FUELTANK_DENSITY * radius * length
            }
            &Part::Engine {radius, length, group} => {
                let _group = group;
                ENGINE_DENSITY * radius * length * 0.5
            }
        }
    }

    fn object (&self) -> RigidBody {
        match self {
            &Part::Vessel {width, length} => {
                let geom = Cuboid::new(Vec2::new(width, length));
                RigidBody::new_dynamic(geom, VESSEL_DENSITY, 1.0, 1.0)
            }
            &Part::FuelTank {radius, length} => {
                let geom = Cylinder::new(length, radius);
                RigidBody::new_dynamic(geom, FUELTANK_DENSITY, 1.0, 1.0)
            }
            &Part::Engine {radius, length, group} => {
                let _group = group;
                let geom = Cone::new(length, radius);
                RigidBody::new_dynamic(geom, ENGINE_DENSITY, 1.0, 1.0)
            }
        }
    }

    fn point_mass (&self) -> PointMass {
        PointMass { center: Vec2::new(0.0,0.0), mass: self.mass() }
    }

}

#[test]
fn simple_parts () {

    let p1 = Part::Vessel { width: 2.0, length: 4.0 };
    let _p2 = Part::FuelTank { radius: 1.0, length: 5.0};
    let _p3 = Part::Engine { radius: 1.0, length: 2.0, group: 3};

    let _m1 = p1.mass();
    let _pm1 = p1.point_mass();

}

struct PartObjectCache {
    part: Part,
    object: RigidBody,
    mass: f64
}

struct Beam {
    length: f64
}

impl Beam {

    fn object (&self) -> RigidBody {
        let geom = Cuboid::new(Vec2::new(BEAM_WIDTH, self.length));
        RigidBody::new_dynamic(geom, BEAM_DENSITY, 1.0, 1.0)
    }
    
    fn mass (&self) -> f64 {
        self.length * BEAM_DENSITY
    }

    fn point_mass (&self) -> PointMass {
        let midpoint = self.length / 2.0;
        PointMass { center: Vec2::new(midpoint, 0.0), mass: self.mass() }
    }

}

struct Attach {
    location: f64,
    rotation: f64
}

type Structure = tagtree::TagTree<Part,Beam,Attach>;

impl Structure {

    fn part_masses(part: &Part) -> Vec<PointMass> {
        // Performance ?
        let mut v = Vec::new();
        v.push(part.point_mass());
        v
    }

    fn beam_masses<'a>(beam: &'a Beam, subs: Vec<&'a Vec<&'a PointMass>>) -> Vec<&'a PointMass> {
        let mut v = Vec::new();
        v.push(beam.point_mass());
        // TODO push all
        v
    }

    fn attach_masses<'a>(attach: &'a Attach, subs: Vec<&'a PointMass>) -> Vec<&'a PointMass> {
        subs
    }

}

fn part(attrs: Part) -> Structure {
    tagtree::TagTree::Leaf(attrs)
}

fn beam(length: f64, parts: Vec<(Attach, Box<Structure>)>) -> Structure {
    tagtree::TagTree::Node(Beam {length: length}, parts)
}
 
#[test]
fn simple_structures () {
    part(Part::Vessel { width: 2.0, length: 4.0 });
    part(Part::FuelTank { radius: 1.0, length: 5.0});
    part(Part::Engine { radius: 1.0, length: 2.0, group: 3});
    beam(5.0, vec![]);
}

#[test]
fn simple_ships () {
    beam(8.0, vec![
         (Attach {location: 2.0, rotation: 0.0}, 
          box part(Part::Vessel {width: 2.0, length: 4.0})),
          (Attach {location: 6.0, rotation: 0.0},
           box part(Part::FuelTank {radius: 1.0, length: 1.5})),
           (Attach {location: 8.0, rotation: 0.0},
            box part(Part::Engine {radius: 1.5, length: 0.5, group: 1}))
           ]);
}
