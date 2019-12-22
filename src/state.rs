use cgmath::prelude::*;
use hecs::World;

use crate::math::{
    Pnt2,
    Rad,
};


pub struct State {
    pub world: World,
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();

        world.spawn((Body::new(),));

        Self {
            world,
        }
    }

    pub fn update(&mut self) {
        for (_, (body,)) in &mut self.world.query::<(&mut Body,)>() {
            body.dir += Rad::full_turn() * 0.01;
        }
    }
}


pub struct Body {
    pub pos: Pnt2,
    pub dir: Rad,
}

impl Body {
    pub fn new() -> Self {
        Self {
            pos: Pnt2::new(0.0, 0.0),
            dir: Rad::zero(),
        }
    }
}
