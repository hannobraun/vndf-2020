use cgmath::prelude::*;
use hecs::World;

use crate::{
    input::Input,
    math::{
        Pnt2,
        Rad,
        Vec2,
        rotate,
    },
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

    pub fn update(&mut self, frame_time: f32, input: &Input) {
        let rotation = input.rotation as i32 as f32;

        for (_, (body,)) in &mut self.world.query::<(&mut Body,)>() {
            body.dir += Rad::turn_div_2() * rotation * frame_time;

            if input.thrust {
                let vel = rotate(Vec2::unit_x(), body.dir) * 300.0;
                body.pos += vel * frame_time;
            }
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
