
use ecs::system::{Process, System};
use ecs::world::{DataHelper};
use nphysics::object::RigidBodyHandle;
use nphysics::world::World;

use sim::{JmpComponents, JmpServices};

pub struct PhysicsHandle {
    pub handle: RigidBodyHandle
}

pub struct PhysicsSystem {
    world: World
}

impl PhysicsSystem {

    pub fn new() -> PhysicsSystem {
        PhysicsSystem { world: World::new() }
    }

}

impl System for PhysicsSystem {
    type Components = JmpComponents;
    type Services = JmpServices;
}

impl Process for PhysicsSystem {
    fn process(&mut self, data: &mut DataHelper<JmpComponents, JmpServices>) {
        for dt in data.services.dt.into_iter() {
            self.world.step(dt)
        }
    }
}
