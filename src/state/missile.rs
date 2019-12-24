use crate::{
    math::{
        Vec2,
        rotate,
    },
    state::Body,
};


pub struct Missile;

impl Missile {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, body: &mut Body) {
        body.acc = rotate(Vec2::unit_x(), body.dir) * 50.0;
    }
}
