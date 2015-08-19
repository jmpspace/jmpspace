
use num::traits::One;

use na::{Iso2, Rot2, Vec1, Vec2};
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

struct WithDepth<T> {
    depth: usize,
    item: T
}

impl Structure {

    fn mass(&self) -> f64 {
        match self {
            &tagtree::TagTree::Leaf(ref part) => {
                part.mass()
            }
            &tagtree::TagTree::Node(ref beam, _) => {
                beam.mass()
            }
        }
    }

    fn total_mass(&self) -> f64 {
        let mut total_mass: f64 = 0.0;
        for item in self.iter() {
            total_mass += item.structure.mass();
        }
        total_mass
    }

    fn point_masses(&self) -> Vec<PointMass> {
        let mut result = Vec::new();
        for item in self.iter() {
            result.push(PointMass{ center: item.context.translation, mass: item.structure.mass() });
        }
        result
    }

}

// Opportunity to move into tagtree if Attach is a monoid
struct StructureContextItem<'a> {
    context: Iso2<f64>,
    structure: &'a Structure
}

struct StructureIter<'a> {
    contexts: Vec<Iso2<f64>>,
    work: Vec<WithDepth<&'a Structure>>
}

impl Structure {
    pub fn iter(&self) -> StructureIter {
        let work = vec!(WithDepth { depth: 0, item: self });
        StructureIter { contexts: vec!(Iso2::one()), work: work }
    }
}

impl<'a> Iterator for StructureIter<'a> {

    type Item = StructureContextItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {

        match self.work.pop() {
            None => { None }
            Some(ref curr_work) => { 
                let context = self.contexts[curr_work.depth];

                match curr_work.item {
                    &tagtree::TagTree::Leaf(_) => {}
                    &tagtree::TagTree::Node(_, ref attachments) => {
                        attachments.iter().fold((), |_, &(ref attach, ref attachment)| {
                            let trn = Vec2::new(attach.location, 0.0);
                            let rot = Rot2::new(Vec1::new(attach.rotation));
                            let iso = Iso2::new_with_rotmat(trn, rot);
                            let next_context = context * iso;
                            let next_depth = curr_work.depth + 1;
                            if self.contexts.len() < next_depth + 1 {
                                self.contexts.push(next_context);
                            } else {
                                self.contexts[next_depth] = next_context;
                            }
                            self.work.push(WithDepth{depth: next_depth, item: &**attachment});
                        })
                    }
                }

                Some(StructureContextItem { context: context, structure: curr_work.item })

            }
        }

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
    let p1 = part(Part::Vessel { width: 2.0, length: 4.0 });
    let a1 = Attach{location: 2.0, rotation: 0.0};
    let p2 = part(Part::FuelTank { radius: 1.0, length: 5.0});
    let a2 = Attach{location: 8.0, rotation: 0.0};
    let p3 = part(Part::Engine { radius: 1.0, length: 2.0, group: 3});
    let a3 = Attach{location: 10.0, rotation: 0.0};
    let b1 = beam(5.0, vec![(a1, box p1),(a2, box p2),(a3, box p3)]);
    b1.mass();
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
