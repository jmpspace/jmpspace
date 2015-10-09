
use ecs::system::{Process, System};
use ecs::world::{DataHelper};
use nphysics::object::RigidBodyHandle;
use nphysics::world::World;

use sim::{JmpComponents, JmpServices};

pub struct PhysicsHandle {
    pub handle: RigidBodyHandle
}

pub struct PhysicsService {
    pub world: World
}

impl PhysicsService {
    pub fn new() -> PhysicsService {
        PhysicsService { world: World::new() }
    }
}

pub struct PhysicsSystem;

impl PhysicsSystem {
    pub fn new() -> PhysicsSystem {
        PhysicsSystem
    }
}

impl System for PhysicsSystem {
    type Components = JmpComponents;
    type Services = JmpServices;
}

impl Process for PhysicsSystem {
    fn process(&mut self, data: &mut DataHelper<JmpComponents, JmpServices>) {
        for dt in data.services.dt.into_iter() {
            println!("Physics step with dt={}", dt);
            data.services.physics.world.step(dt)
        }
    }
}
