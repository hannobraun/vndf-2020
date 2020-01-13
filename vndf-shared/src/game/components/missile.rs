use std::f32;

use pid::Pid;
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
        Rad,
    },
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Missile {
    pub target:   Pnt2,
    pub guidance: Pid<f32>,
}

impl Missile {
    pub fn new(target: Pnt2) -> Self {
        let guidance = Pid::new(
            // Proportional term
            0.01,
            // Integral term
            0.0,
            // Derivative term
            0.0,
            // Limit of proportional output
            Rad::full_turn().0,
            // Limit of integral output
            0.0,
            // Limit of derivative output
            0.0,
            // Set point
            0.0,
        );

        Self {
            target,
            guidance,
        }
    }

    pub fn update_guidance(&mut self, body: &mut Body) {
        let to_target = self.target - body.pos;
        let angle     = to_target.angle(body.vel);

        body.dir.0 += self.guidance.next_control_output(angle.0).output;
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
