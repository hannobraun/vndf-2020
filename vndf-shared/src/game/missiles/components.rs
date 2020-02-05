use pid::Pid;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    cgs::{
        Handle,
        Store,
    },
    game::{
        crafts::Craft,
        health::Health,
        physics::{
            Body,
            Position,
            Velocity,
        },
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
    pub craft:    Handle,
    pub guidance: Handle,
    pub target:   Handle,
}

impl Missile {
    pub fn new(
        craft:    Handle,
        guidance: Handle,
        target:   Handle,
    )
        -> Self
    {
        Self {
            craft,
            guidance,
            target,
        }
    }

    pub fn remove(
        handle:     Handle,
        bodies:     &mut Store<Body>,
        crafts:     &mut Store<Craft>,
        guidances:  &mut Store<Guidance>,
        healths:    &mut Store<Health>,
        missiles:   &mut Store<Missile>,
        positions:  &mut Store<Position>,
        targets:    &mut Store<Target>,
        velocities: &mut Store<Velocity>,
    )
        -> Option<()>
    {
        let missile = missiles.remove(handle)?;

        Craft::remove(
            missile.craft,
            bodies,
            crafts,
            healths,
            positions,
            velocities,
        );
        guidances.remove(missile.guidance);
        targets.remove(missile.target);

        Some(())
    }
}


pub struct Guidance {
    pub craft:    Handle,
    pub target:   Handle,
    pub guidance: Pid<f32>,
}

impl Guidance {
    pub fn new(craft: Handle, target: Handle) -> Self {
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
            craft,
            target,
            guidance,
        }
    }

    pub fn update_guidance(&mut self,
        bodies:     &mut Store<Body>,
        crafts:     &Store<Craft>,
        positions:  &Store<Position>,
        targets:    &Store<Target>,
        velocities: &Store<Velocity>,
    )
        -> Option<()>
    {
        let craft  = crafts.get(self.craft)?;
        let target = targets.get(self.target)?;
        let body   = bodies.get_mut(craft.body)?;
        let pos    = positions.get(body.pos)?;
        let vel    = velocities.get(body.vel)?;

        let to_target = target.value - pos.0;

        let projection = vel.0.project_on(to_target);
        let rejection  = vel.0 - projection;

        let error_dir = {
            let cross = to_target.extend(0.0).cross(rejection.extend(0.0));
            match cross.z {
                dir if dir >  0.0 =>  1.0,
                dir if dir <  0.0 => -1.0,
                dir if dir == 0.0 =>  0.0,

                // The above cover all the regular cases, but if the missile
                // sits directly on top of the target, we'll get `NaN`. Doesn't
                // really matter what we do here in this case, so let's just
                // give it a valid value.
                _ => 0.0,
            }
        };
        let error = rejection.magnitude() * error_dir;

        let control_output = self.guidance.next_control_output(error);
        body.dir = rotate(to_target, cgmath::Rad(control_output.output));

        Some(())
    }

    pub fn explode_if_ready(&self,
        bodies:    &Store<Body>,
        crafts:    &Store<Craft>,
        healths:   &mut Store<Health>,
        positions: &Store<Position>,
        targets:   &Store<Target>,
    )
        -> Option<()>
    {
        let     craft  = crafts.get(self.craft)?;
        let     target = targets.get(self.target)?;
        let     body   = bodies.get(craft.body)?;
        let     pos    = positions.get(body.pos)?;
        let mut health = healths.get_mut(craft.health)?;

        let no_fuel_left   = craft.fuel <= 0.0;
        let near_target    = (pos.0 - target.value).magnitude() <= 10.0;
        let should_explode = no_fuel_left || near_target;

        if should_explode {
            health.value = 0.0;
        }

        Some(())
    }
}


#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct Target {
    pub craft: Handle,
    pub value: Pnt2,
}

impl Target {
    pub fn update(&mut self,
        crafts:  &Store<Craft>,
        targets: impl IntoIterator<Item=(Position, Craft)>,
    )
        -> Option<()>
    {
        let craft = crafts.get(self.craft)?;

        let mut best_rating = 0.0;
        let mut new_target  = None;

        for (target_pos, target_craft) in targets {
            if target_craft.owner == craft.owner {
                continue;
            }

            let distance  = (self.value - target_pos.0).magnitude();
            let threshold = 100.0;
            let rating    = 1.0 / (threshold - distance);

            if rating > best_rating {
                best_rating = rating;
                new_target  = Some(target_pos.0);
            }
        }

        if let Some(new_target) = new_target {
            self.value = new_target
        }

        Some(())
    }
}
