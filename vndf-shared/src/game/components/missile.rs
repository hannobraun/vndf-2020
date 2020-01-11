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

    pub fn update_guidance(&self, body: &mut Body) {
        let to_target = self.target - body.pos;
        body.dir = Vec2::unit_x().angle(to_target);
    }

    pub fn should_explode(&self, body: &Body, engine: &Engine)
        -> Option<e::Explosion>
    {
        let no_fuel_left = engine.fuel <= 0.0;
        let near_target  = (body.pos - self.target).magnitude() <= 10.0;

        if no_fuel_left || near_target {
            Some(e::explosion(body))
        }
        else {
            None
        }
    }
}
