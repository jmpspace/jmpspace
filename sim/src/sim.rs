
use nphysics::world::World;

pub struct Sim {
    pub a: i32,
    pub world: World,
}

impl Sim {

    pub fn new(a: i32) -> Sim {
        
        let world = World::new();
        Sim { a: a, world: world }

    }

}

