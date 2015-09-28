
use ecs::{BuildData, DataHelper, EntityIter, ServiceManager, System, World};
use ecs::system::entity::{EntityProcess, EntitySystem};
use protobuf::repeated::RepeatedField;
use protobuf::core::Message;

use contracts::actions::Action;
use contracts::ship as shipTracts; // TODO move out for this reason
use contracts::world::Snapshot;

use demo::simple_ship;
use ship::Structure;
use physics::{PhysicsHandle, PhysicsService, PhysicsSystem};

components! {
    struct JmpComponents {
        #[cold] structure: Structure,
        #[hot] physics_handle: PhysicsHandle
    }
}

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

    pub fn connect(&mut self, client: i32) -> u64 {
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
        id
    }

    pub fn apply(&mut self, client: i32, action: &Action) {
        println!("Apply {} {:?}", client, action);
    }

    pub fn apply_buf(&mut self, client: i32, buf: Vec<u8>) -> i32 {
        let mut action = Action::new();
        if let Err(_) = action.merge_from_bytes(buf.as_slice()) {
            // TODO logging
            // TODO meaningful error codes in header file
            return 1;
        }
        self.apply(client, &action);
        0
    }

    pub fn snapshot(&mut self) -> Snapshot {
        self.world.update();
        self.world.services.snapshot.clone().expect("Should see a snapshot")
    }

    pub fn snapshot_buf(&mut self) -> Vec<u8> {
        let snapshot = self.snapshot();
        let mut snapshot_vec = Vec::new();
        if let Err(_) = snapshot.write_to_vec(&mut snapshot_vec) {
            // TODO logging
            // TODO meaningful error code
        }
        snapshot_vec
    }

}

#[test]
fn simple_sim () {
    let mut sim = Sim::new();
    let entity = sim.connect(121642);
    println!("Entity {}", entity);
}
