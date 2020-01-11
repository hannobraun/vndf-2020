use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    game::{
        components::{
            Body,
            Engine,
        },
        entities as e,
    },
    math::{
        prelude::*,
        Pnt2,
        Vec2,
    },
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Missile {
    pub target: Pnt2,
}

impl Missile {
    pub fn new(target: Pnt2) -> Self {
        Self {
            target,
        }
    }

    pub fn update(&self, body: &mut Body, engine: &Engine)
        -> Option<e::Explosion>
    {
        let to_target = self.target - body.pos;
        body.dir = Vec2::unit_x().angle(to_target);

        if engine.fuel <= 0.0 {
            Some(e::explosion(body))
        }
        else {
            None
        }
    }
}
