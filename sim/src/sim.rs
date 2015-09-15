
use ecs::{BuildData, World};
use ecs::world::{ServiceManager};

use contracts::actions::Action;
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
    pub physics: PhysicsService
}

impl Default for JmpServices {
    fn default() -> Self {
        JmpServices {
            dt: None,
            physics: PhysicsService::new()
        }
    }
}

impl ServiceManager for JmpServices {}

systems! {
    struct JmpSystems<JmpComponents, JmpServices> {
        physics: PhysicsSystem = PhysicsSystem::new()
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
        let body = ship.rigid_body();
        let rb_handle = self.world.services.physics.world.add_body(body);
        let physics_handle = PhysicsHandle { handle: rb_handle };
        let entity = self.world.create_entity(
            |entity: BuildData<JmpComponents>, data: &mut JmpComponents| {
                data.structure.add(&entity, ship);
                data.physics_handle.add(&entity, physics_handle);
            });
        entity.id()
    }

    pub fn update(&mut self) {
        self.world.update();
    }

    pub fn apply(&mut self, client: i32, action: &Action) {
        println!("Apply {} {:?}", client, action);
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot::new()
    }

}

