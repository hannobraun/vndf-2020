use std::net::SocketAddr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    game::{
        components::Body,
        entities as e,
    },
    input::Rotation,
    math::{
        prelude::*,
        Rad,
    },
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Ship {
    pub rotation: Rotation,
    pub missiles: u64,
    pub player:   SocketAddr,
}

impl Ship {
    pub fn new(player: SocketAddr) -> Self {
        Self {
            rotation: Rotation::None,
            missiles: 16,
            player,
        }
    }

    pub fn launch_missile(&mut self, body: &Body) -> Option<e::Missile> {
        if self.missiles > 0 {
            self.missiles -= 1;
            Some(e::missile(body))
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
