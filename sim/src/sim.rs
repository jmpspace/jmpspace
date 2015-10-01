
use na::{Vec1};
use ecs::{BuildData, DataHelper, Entity, EntityIter, EntityModifier, ServiceManager, System, World};
use ecs::system::entity::{EntityProcess, EntitySystem};
use protobuf::repeated::RepeatedField;
use protobuf::core::Message;

use contracts::actions::{Action, Controls};
use contracts::ship as shipTracts; // TODO move out for this reason
use contracts::world::{GameUpdate, Snapshot};

use demo::simple_ship;
use ship::{Structure, ThrustProfile};
use physics::{PhysicsHandle, PhysicsService, PhysicsSystem};

components! {
    struct JmpComponents {
        #[cold] structure: Structure,
        #[hot] physics_handle: PhysicsHandle
    }
}

struct ControlsModifier<'a> {
    pub controls: &'a Controls
}

/*
impl<'a> EntityModifier<JmpComponents> for ControlsModifier<'a> {
    fn modify
}
*/

pub struct JmpServices {
    pub dt: Option<f64>,
    pub physics: PhysicsService,
    pub snapshot: Option<Snapshot>
}

impl Default for JmpServices {
    fn default() -> Self {
        JmpServices {
            dt: None,
            physics: PhysicsService::new(),
            snapshot: None
        }
    }
}

impl ServiceManager for JmpServices {}

// I guess move this out too... TODO
pub struct SnapshotProcess;

impl System for SnapshotProcess { type Components = JmpComponents; type Services = JmpServices; }

impl EntityProcess for SnapshotProcess {
    fn process(&mut self, entities: EntityIter<JmpComponents>, data: &mut DataHelper<JmpComponents, JmpServices>) {
        let mut snapshot = Snapshot::new();
        let mut ships: Vec<shipTracts::Ship> = Vec::new();
        for e in entities {
            let mut ship = shipTracts::Ship::new();
            ship.set_entityId(e.id());
            let structure = data.structure[e].contract();
            ship.set_structure(structure);
            ships.push(ship);
        }
        println!("Serializing {} structures in snapshot", ships.len());
        snapshot.set_ships(RepeatedField::from_vec(ships));
        data.services.snapshot = Some(snapshot);
    }
}

systems! {
    struct JmpSystems<JmpComponents, JmpServices> {
        physics: PhysicsSystem = PhysicsSystem::new(),
        snapshot: EntitySystem<SnapshotProcess> = EntitySystem::new(
            SnapshotProcess,
            aspect!(<JmpComponents> all: [structure])
            )
    }
}

pub struct Sim {
    world: World<JmpSystems>
}

impl Sim {

    pub fn new() -> Sim {
        
        let world = World::<JmpSystems>::new();
        Sim { world: world }

    }

    pub fn connect(&mut self, client: i32) -> (Entity, Vec<u8>) {
        println!("Creating a client {}", client);
        let ship = simple_ship();
        println!("Created ship");
        let body = ship.rigid_body();
        println!("Calculated ship body");
        let rb_handle = self.world.services.physics.world.add_body(body);
        println!("Added body to physics, got handle");
        let physics_handle = PhysicsHandle { handle: rb_handle };
        let entity = self.world.create_entity(
            |entity: BuildData<JmpComponents>, data: &mut JmpComponents| {
                data.structure.add(&entity, ship);
                data.physics_handle.add(&entity, physics_handle);
            });
        let id = entity.id();
        println!("Created an entity {}", id);
        let mut game_update = GameUpdate::new();
        game_update.set_focusEntityId(id);
        let mut game_update_vec = Vec::new();
        if let Err(_) = game_update.write_to_vec(&mut game_update_vec) {
            // TODO logging
            // TODO meaningful error code
        }
        (entity, game_update_vec)
    }

    pub fn apply(&mut self, entity: Entity, action: &Action) {
        println!("Apply {} {:?}", entity.id(), action);
        if action.has_controls() {
            let controls = action.get_controls();
            if controls.has_brakes() {
                println!("I cut the brakes, wildcard!"); // TODO
            }
            if controls.has_active() {
                let active_groups = controls.get_active().get_groups();
                self.world.with_entity_data(&entity, |entity, data| {
                    let profiles = data.structure[entity].thrust_profiles();
                    // TODO formatting
                    let net_profile = active_groups.iter().fold(ThrustProfile::zero(), |mut acc, group|
                                                                {
                                                                    if let Some(profile) = profiles.get(group) {
                                                                        acc.add(profile)
                                                                    }
                                                                    acc
                                                                });
                    let ref mut body = data.physics_handle[entity].handle.borrow_mut();
                    body.clear_forces();
                    body.append_lin_force(net_profile.force);
                    body.append_ang_force(Vec1::new(net_profile.torque));
                });
            }
        }
    }

    pub fn apply_buf(&mut self, entity: Entity, buf: Vec<u8>) -> i32 {
        let mut action = Action::new();
        if let Err(_) = action.merge_from_bytes(buf.as_slice()) {
            // TODO logging
            // TODO meaningful error codes in header file
            return 1;
        }
        self.apply(entity, &action);
        0
    }

    pub fn snapshot(&mut self) -> Snapshot {
        self.world.update();
        self.world.services.snapshot.clone().expect("Should see a snapshot")
    }

    pub fn snapshot_buf(&mut self) -> Vec<u8> {
        let mut game_update = GameUpdate::new();
        let snapshot = self.snapshot();
        game_update.set_snapshot(snapshot);
        let mut game_update_vec = Vec::new();
        if let Err(_) = game_update.write_to_vec(&mut game_update_vec) {
            // TODO logging
            // TODO meaningful error code
        }
        game_update_vec
    }

}

#[test]
fn simple_sim () {
    let mut sim = Sim::new();
    let entity = sim.connect(121642);
    println!("Entity {}", entity);
}
