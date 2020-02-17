use pid::Pid;
use serde::{
    Deserialize,
    Serialize,
};
use toadster::{
    Handle,
    handle,
    store,
};

use crate::{
    game::{
        crafts::{
            Craft,
            Fuel,
        },
        health::Health,
        physics::{
            Body,
            Direction,
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


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Missile {
    pub craft:    Handle<Craft>,
    pub guidance: Handle<Guidance>,
    pub target:   Handle<Target>,
}

impl Missile {
    pub fn new(
        craft:    impl Into<Handle<Craft>>,
        guidance: impl Into<Handle<Guidance>>,
        target:   impl Into<Handle<Target>>,
    )
        -> Self
    {
        Self {
            craft:    craft.into(),
            guidance: guidance.into(),
            target:   target.into(),
        }
    }

    pub fn remove(
        handle:     handle::Strong<Missile>,
        bodies:     &mut store::Strong<Body>,
        crafts:     &mut store::Strong<Craft>,
        directions: &mut store::Strong<Direction>,
        fuels:      &mut store::Strong<Fuel>,
        guidances:  &mut store::Strong<Guidance>,
        missiles:   &mut store::Strong<Missile>,
        positions:  &mut store::Strong<Position>,
        targets:    &mut store::Strong<Target>,
        velocities: &mut store::Strong<Velocity>,
    )
        -> Option<()>
    {
        let missile = missiles.remove(handle)?;

        Craft::remove(
            missile.craft.strong(),
            bodies,
            crafts,
            directions,
            fuels,
            positions,
            velocities,
        );
        guidances.remove(missile.guidance.strong());
        targets.remove(missile.target.strong());

        Some(())
    }
}


pub struct Guidance {
    pub craft:    handle::Strong<Craft>,
    pub target:   handle::Strong<Target>,
    pub guidance: Pid<f32>,
}

impl Guidance {
    pub fn new(
        craft: handle::Strong<Craft>,
        target: handle::Strong<Target>,
    )
        -> Self
    {
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
        bodies:     &mut store::Strong<Body>,
        crafts:     &store::Strong<Craft>,
        directions: &mut store::Strong<Direction>,
        positions:  &store::Strong<Position>,
        targets:    &store::Strong<Target>,
        velocities: &store::Strong<Velocity>,
    )
        -> Option<()>
    {
        let craft  = crafts.get(&self.craft)?;
        let target = targets.get(&self.target)?;
        let body   = bodies.get_mut(&craft.body)?;
        let pos    = positions.get(&body.pos)?;
        let vel    = velocities.get(&body.vel)?;
        let dir    = directions.get_mut(&body.dir)?;

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
        dir.0 = rotate(to_target, cgmath::Rad(control_output.output));

        Some(())
    }

    pub fn explode_if_ready(&self,
        bodies:    &store::Strong<Body>,
        crafts:    &store::Strong<Craft>,
        fuels:     &store::Strong<Fuel>,
        healths:   &mut store::Strong<Health>,
        positions: &store::Strong<Position>,
        targets:   &store::Strong<Target>,
    )
        -> Option<()>
    {
        let     craft  = crafts.get(&self.craft)?;
        let     target = targets.get(&self.target)?;
        let     body   = bodies.get(&craft.body)?;
        let     pos    = positions.get(&body.pos)?;
        let mut health = healths.get_mut(&craft.health)?;
        let     fuel   = fuels.get(&craft.fuel)?;

        let no_fuel_left   = fuel.0 <= 0.0;
        let near_target    = (pos.0 - target.value).magnitude() <= 10.0;
        let should_explode = no_fuel_left || near_target;

        if should_explode {
            health.value = 0.0;
        }

        Some(())
    }
}


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Target {
    pub craft: Handle<Craft>,
    pub value: Pnt2,
}

impl Target {
    pub fn update(&mut self,
        crafts:  &store::Strong<Craft>,
        targets: impl IntoIterator<Item=(Position, Craft)>,
    )
        -> Option<()>
    {
        let craft = crafts.get(&self.craft)?;

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
