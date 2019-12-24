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


pub const WORLD_SIZE: f32 = 1000.0;


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
        for (_, (body,)) in &mut self.world.query::<(&mut Body,)>() {
            let rotation = input.rotation as i32 as f32;
            body.dir += Rad::turn_div_2() * rotation * frame_time;

            body.acc = if input.thrust {
                rotate(Vec2::unit_x(), body.dir) * 300.0
            }
            else {
                Vec2::zero()
            };

            body.update(frame_time);
        }
    }
}


pub struct Body {
    pub pos: Pnt2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub dir: Rad,
}

impl Body {
    pub fn new() -> Self {
        Self {
            pos: Pnt2::new(0.0, 0.0),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
            dir: Rad::zero(),
        }
    }

    pub fn update(&mut self, frame_time: f32) {
        self.vel += self.acc * frame_time;
        self.pos += self.vel * frame_time;
    }
}
