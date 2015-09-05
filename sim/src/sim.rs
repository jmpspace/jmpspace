
use ecs::{World};
use ecs::world::{ServiceManager};

use physics::{PhysicsHandle, PhysicsSystem};

components! {
    struct JmpComponents {
        #[hot] physics_handle: PhysicsHandle
    }
}

pub struct JmpServices {
    pub dt: f64
}

impl Default for JmpServices {
    fn default() -> Self {
        JmpServices {
            dt: f64::default()
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
    pub a: i32,
    world: World<JmpSystems>
}

impl Sim {

    pub fn new(a: i32) -> Sim {
        
        let world = World::<JmpSystems>::new();
        Sim { a: a, world: world }

    }

}

