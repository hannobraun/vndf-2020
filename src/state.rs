use hecs::World;

use crate::math::Pnt2;


pub struct State {
    pub world: World,
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();

        world.spawn((Pnt2::new(0.0, 0.0),));

        Self {
            world,
        }
    }
}
