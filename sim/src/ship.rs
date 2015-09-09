
use std::boxed::Box;
use std::ops::Neg;
use std::sync::Arc;
use num::traits::{One, Zero};

use na::{Iso2, Mat1, Norm, Rot2, Translation, Vec1, Vec2};
use ncollide::inspection::{Repr2};
use ncollide::shape::{Compound, Cone, Cuboid, Cylinder};
use nphysics::object::{RigidBody};
use protobuf::repeated::RepeatedField;

use constants::*;
use contracts::ship as contracts;
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

type Shaped = Box<Repr2<f64>>;
type ArcShaped = Arc<Shaped>;

impl PointMass {

    fn mass_contrib (&self) -> Vec2<f64> {
        self.center * self.mass
    }

    fn moment_contrib (&self) -> f64 {
        self.mass * self.center.sqnorm()
    }

}

impl Part {

    fn contract (&self) -> contracts::Part {
        let mut part = contracts::Part::new();
        match self {
            &Part::Vessel { width, length } => {
                let mut vessel = contracts::Vessel::new();
                vessel.set_width(width);
                vessel.set_length(length);
                part.set_vessel(vessel);
            }
            &Part::FuelTank {radius, length} => {
                let mut fueltank = contracts::FuelTank::new();
                fueltank.set_radius(radius);
                fueltank.set_length(length);
                part.set_fuelTank(fueltank);
            }
            &Part::Engine {radius, length, group} => {
                let mut engine= contracts::Engine::new();
                engine.set_radius(radius);
                engine.set_length(length);
                engine.set_group(group);
                part.set_engine(engine);
            }
        }
        part
    }

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


    fn geometry (&self) -> ArcShaped {
        Arc::new(match self {
            &Part::Vessel {width, length} => {
                Box::new(Cuboid::new(Vec2::new(width, length))) as Shaped
            }
            &Part::FuelTank {radius, length} => {
                Box::new(Cylinder::new(length, radius)) as Shaped
            }
            &Part::Engine {radius, length, group} => {
                let _group = group;
                Box::new(Cone::new(length, radius)) as Shaped
            }
        })
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

    fn contract (&self) -> contracts::Beam {
        let mut beam = contracts::Beam::new();
        beam.set_length(self.length);
        beam
    }

    fn geometry (&self) -> ArcShaped {
        let geom = Cuboid::new(Vec2::new(BEAM_WIDTH, self.length));
        Arc::new(Box::new(geom))
    }

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

#[derive(Clone)]
struct Attach {
    location: f64,
    rotation: f64
}

type Structure = tagtree::TagTree<Part,Beam,Attach>;

impl Structure {

    fn contract(&self) -> contracts::Structure {
        let mut structure = contracts::Structure::new();
        // performance!
        let mut data: Vec<contracts::StructureData> = Vec::new();
        let mut counter = 0;
        for item in self.attach_iter() {
            match item {
                StructureData::Marker => {
                    data.push(contracts::EndMarker::New())
                }
                StructureData::Node(item) => {
                    let mut node = contracts::StructureNode::new();
                    counter += 1;
                    item.context.set_link(node);
                    item.structure.set_node(node);
                    node.set_attachment(item.structure.contract_attachment());
                    data.push(node)
                }
            }
        }
        structure.set_structure(RepeatedField::from_vec(data));
        structure
    }

    fn contract_node(&self, builder: &mut contracts::StructureNode) {
        match self {
            &tagtree::TagTree::Leaf(ref part) => {
                builder.set_part(part.contract())
            }
            &tagtree::TagTree::Node(ref beam, _) => {
                builder.set_beam(beam.contract())
            }
        }
    }

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

    fn geometry (&self) -> ArcShaped {
        match self {
            &tagtree::TagTree::Leaf(ref part) => {
                part.geometry()
            }
            &tagtree::TagTree::Node(ref beam, _) => {
                beam.geometry()
            }
        }
    }

    fn total_mass(&self) -> f64 {
        let mut total_mass: f64 = 0.0;
        for item in self.iso_iter() {
            total_mass += item.structure.mass();
        }
        total_mass
    }

    fn point_masses(&self) -> Vec<PointMass> {
        // This smells, could probably return an iterator?
        self.iso_iter().fold(Vec::new(), |mut acc, item| {
            acc.push(PointMass{ center: item.context.translation, mass: item.structure.mass() });
            acc
        })
    }

    fn center_of_mass(&self) -> Vec2<f64> {
        self.point_masses().iter().fold(Vec2::zero(), |acc, pm| {
            acc + pm.mass_contrib()
        })
    }

    fn total_moment(&self) -> f64 {
        let mut point_masses = self.point_masses();
        let neg_center = self.center_of_mass().neg();
        for mut pm in point_masses.iter_mut() {
            pm.center.append_translation_mut(&neg_center);
        }
        point_masses.iter().fold(0.0, |acc, pm| {
            acc + pm.moment_contrib()
        })
    }

    fn compound_shape(&self) -> ArcShaped {
        let mut acc = Vec::<(Iso2<f64>,ArcShaped)>::new();
        for item in self.iso_iter() {
            acc.push((item.context, item.structure.geometry()));
        }
        Arc::new(Box::new(Compound::new(acc)))
    }

    fn rigid_body(&self) -> RigidBody {
        let shape = self.compound_shape();
        let mass_properties = Some((self.total_mass(), self.center_of_mass().to_pnt(), Mat1::new(self.total_moment())));
        RigidBody::new(shape, mass_properties, 1.0, 1.0)
    }
}

// Opportunity to move into tagtree if Attach is a monoid
struct StructureWorkItem<'a, T> {
    context: T,
    structure: &'a Structure,
}

struct StructureIsoIter<'a> {
    work: Vec<StructureWorkItem<'a, Iso2<f64>>>,
}

enum StructureLink {
    Root, Attach(Attach)
}

impl StructureLink {

    fn contract_link(&self, builder: &mut contracts::StructureNode) {
        match self {
            StructureLink::Root => {
                builder.set_root(contracts::Root::new())
            }
            StructureLink::Attach(attach) => {
                builder.set_attach(attach)
            }
        }
    }

}

struct StructureAttachIter<'a> {
    work: Vec<StructureWorkItem<'a, StructureLink>>,
}

impl Structure {
    pub fn iso_iter(&self) -> StructureIsoIter {
        let root_work_item = StructureWorkItem {
            context: Iso2::one(),
            structure: self,
        };
        StructureIsoIter { work: vec!(root_work_item) }
    }

    // Not very DRY, not exactly sure where to abstract
    pub fn attach_iter(&self) -> StructureAttachIter {
        let root_work_item = StructureWorkItem {
            context: None,
            structure: self,
        };
        StructureAttachIter { work: vec!(root_work_item) }
    }
}

impl<'a> Iterator for StructureIsoIter<'a> {

    type Item = StructureWorkItem<'a, Iso2<f64>>;

    fn next(&mut self) -> Option<Self::Item> {

        match self.work.pop() {
            None => { None }
            Some(ref curr_work) => { 
                let context = curr_work.context;

                match curr_work.structure {
                    &tagtree::TagTree::Leaf(_) => {}
                    &tagtree::TagTree::Node(_, ref attachments) => {
                        // This smells, should probably rewrite as a normal for loop
                        attachments.iter().fold((), |_, &(ref attach, ref attachment)| {
                            let trn = Vec2::new(attach.location, 0.0);
                            let rot = Rot2::new(Vec1::new(attach.rotation));
                            let iso = Iso2::new_with_rotmat(trn, rot);
                            let next_context = context * iso;
                            let next_work = StructureWorkItem { 
                                context: next_context, 
                            structure: &**attachment
                            };

                            self.work.push(next_work);
                        })
                    }
                }

                Some(StructureWorkItem { context: context, structure: curr_work.structure })

            }
        }

    }

}

enum StructureData<T> {
    Marker,
    Node(T)
}

impl<'a> Iterator for StructureAttachIter<'a> {

    type Item = StructureData<StructureWorkItem<'a, Option<Attach>>>;

    fn next(&mut self) -> Option<Self::Item> {

        match self.work.pop() {
            None => { None }
            Some(ref curr_work) => { 
                let context = curr_work.context.clone();

                match curr_work.structure {
                    &tagtree::TagTree::Leaf(_) => {}
                    &tagtree::TagTree::Node(_, ref attachments) => {
                        // This smells, should probably rewrite as a normal for loop
                        attachments.iter().fold((), |_, &(ref attach, ref attachment)| {
                            //let trn = Vec2::new(attach.location, 0.0);
                            //let rot = Rot2::new(Vec1::new(attach.rotation));
                            //let iso = Iso2::new_with_rotmat(trn, rot);
                            //let next_context = context * iso;
                            let next_context = Some(attach.clone());
                            let next_work = StructureWorkItem { 
                                context: next_context, 
                            structure: &**attachment,
                            };

                            self.work.push(StructureData::Node(next_work));
                        })
                    }
                }

                self.work.push(StructureData::Marker);

                Some(StructureWorkItem { context: context, structure: curr_work.structure })

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
          box part(Part::Vessel {width: 2.0, length: 4.0})
         ),
         (Attach {location: 6.0, rotation: 0.0},
          box part(Part::FuelTank {radius: 1.0, length: 1.5})
         ),
         (Attach {location: 8.0, rotation: 0.0},
          box part(Part::Engine {radius: 1.5, length: 0.5, group: 1})
         )
         ]
        );
}
