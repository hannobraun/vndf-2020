use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    cgs::Handle,
    game::{
        missiles::MissileEntity,
        physics::Body,
        players::PlayerId,
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
    pub entity: u64,
    pub craft:  Handle,

    pub rotation: Rotation,
    pub missiles: u64,
    pub color:    [f32; 3],
}

impl Ship {
    pub fn new(entity: hecs::Entity, craft: Handle, color: [f32; 3]) -> Self {
        Self {
            entity:   entity.to_bits(),
            craft,
            rotation: Rotation::None,
            missiles: 16,
            color,
        }
    }

    pub fn launch_missile(&mut self, owner: PlayerId, body: &Body, target: Pnt2)
        -> Option<MissileEntity>
    {
        if self.missiles > 0 {
            self.missiles -= 1;
            Some(MissileEntity { owner, origin: *body, target })
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
