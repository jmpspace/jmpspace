
use ecs::{World};
use ecs::world::{ServiceManager};

use contracts::actions::Action;
use contracts::world::Snapshot;

use physics::{PhysicsHandle, PhysicsSystem};

components! {
    struct JmpComponents {
        #[hot] physics_handle: PhysicsHandle
    }
}

pub struct JmpServices {
    pub dt: Option<f64>
}

impl Default for JmpServices {
    fn default() -> Self {
        JmpServices {
            dt: None
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

    pub fn connect(&mut self, client: i32) {
        println!("Creating a client {}", client);
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

