
use na::{Rotation};
use ecs::{DataHelper, EntityIter, System};
use ecs::system::entity::{EntityProcess};
use protobuf::repeated::RepeatedField;

use contracts::actions::{Active};
use contracts::ship::{PhysicsState, Ship}; // TODO move out for this reason
use contracts::world::{Snapshot};

use sim::{JmpComponents, JmpServices};

pub struct SnapshotProcess;

impl System for SnapshotProcess { type Components = JmpComponents; type Services = JmpServices; }

impl EntityProcess for SnapshotProcess {
    fn process(&mut self, entities: EntityIter<JmpComponents>, data: &mut DataHelper<JmpComponents, JmpServices>) {
        let mut snapshot = Snapshot::new();
        let mut ships: Vec<Ship> = Vec::new();
        for e in entities {
            let mut ship = Ship::new();
            ship.set_entityId(e.id());
            let structure = data.structure[e].contract();
            ship.set_structure(structure);
            let ref body = data.physics_handle[e].handle.borrow();
            let mut physics_state = PhysicsState::new();
            physics_state.set_x(body.position().translation.x);
            physics_state.set_y(body.position().translation.y);
            physics_state.set_theta(body.position().rotation.rotation().x);
            physics_state.set_vx(body.lin_vel().x);
            physics_state.set_vy(body.lin_vel().y);
            physics_state.set_omega(body.ang_vel().x);
            ship.set_physicsState(physics_state);
            let mut active = Active::new();
            active.set_groups(data.active_thrusters[e].clone());
            ship.set_active(active);
            ships.push(ship);
        }
        println!("Serializing {} structures in snapshot", ships.len());
        snapshot.set_ships(RepeatedField::from_vec(ships));
        data.services.snapshot = Some(snapshot);
    }
}