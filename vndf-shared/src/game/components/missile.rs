use pid::Pid;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    game::{
        components::{
            Body,
            Craft,
        },
        entities as e,
    },
    math::{
        prelude::*,
        Pnt2,
        Rad,
        rotate,
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
            // Proportional gain
            0.1,
            // Integral gain
            0.0,
            // Derivative gain
            0.0,
            // Proportional limit
            Rad::turn_div_4().0,
            // Integral limit
            0.0,
            // Derivative limit
            0.0,
            // Setpoint
            0.0,
        );

        Self {
            target,
            guidance,
        }
    }

    pub fn update_guidance(&mut self, body: &mut Body) {
        let to_target = self.target - body.pos;

        let projection = body.vel.project_on(to_target);
        let rejection  = body.vel - projection;

        let error_dir = {
            let cross = to_target.extend(0.0).cross(rejection.extend(0.0));
            match cross.z {
                dir if dir  > 0.0 =>  1.0,
                dir if dir  < 0.0 => -1.0,
                dir if dir == 0.0 =>  0.0,

                _ => unreachable!(),
            }
        };
        let error = rejection.magnitude() * error_dir;

        let control_output = self.guidance.next_control_output(error);
        body.dir = rotate(to_target, cgmath::Rad(control_output.output));
    }

    pub fn should_explode(&self, body: &Body, craft: &Craft)
        -> Option<e::Explosion>
    {
        let no_fuel_left = craft.fuel <= 0.0;
        let near_target  = (body.pos - self.target).magnitude() <= 10.0;

        if no_fuel_left || near_target {
            Some(e::explosion(body))
        }
        else {
            None
        }
    }
}
