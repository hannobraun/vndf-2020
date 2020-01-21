use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    game::{
        PlayerId,
        entities as e,
        features::physics::components::Body,
    },
    input::Rotation,
    math::{
        prelude::*,
        Pnt2,
        Rad,
    },
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Ship {
    pub rotation: Rotation,
    pub missiles: u64,
    pub color:    [f32; 3],
}

impl Ship {
    pub fn new(color: [f32; 3]) -> Self {
        Self {
            rotation: Rotation::None,
            missiles: 16,
            color,
        }
    }

    pub fn launch_missile(&mut self, owner: PlayerId, body: &Body, target: Pnt2)
        -> Option<e::Missile>
    {
        if self.missiles > 0 {
            self.missiles -= 1;
            Some(e::missile(owner, body, target))
        }
        else {
            None
        }
    }

    pub fn update(&self, body: &mut Body) {
        let rotation = self.rotation as i32 as f32;
        body.rot = Rad::full_turn() * 0.6 * rotation;
    }
}
