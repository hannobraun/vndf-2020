use log::warn;
use serde::{
    Deserialize,
    Serialize,
};
use toadster::{
    Store,
    StrongHandle,
};
use vndf_events as events;

use crate::{
    game::{
        crafts::{
            Craft,
            Fuel,
        },
        health::Health,
        missiles::{
            MissileEntity,
            MissileLaunch,
        },
        physics::{
            Body,
            Direction,
            Position,
            Velocity,
        },
        players::{
            Player,
            PlayerId,
        },
    },
    input::{
        self,
        Action,
        Rotation,
    },
    math::{
        prelude::*,
        Pnt2,
        Rad,
    },
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Ship {
    pub craft:    StrongHandle<Craft>,
    pub rotation: Rotation,
    pub missiles: u64,
    pub color:    [f32; 3],
}

impl Ship {
    pub fn new(
        craft:  StrongHandle<Craft>,
        color:  [f32; 3],
    ) -> Self {
        Self {
            craft,
            rotation: Rotation::None,
            missiles: 16,
            color,
        }
    }

    pub fn apply_input(&mut self,
        bodies:         &Store<Body>,
        crafts:         &mut Store<Craft>,
        missile_launch: &mut events::Sink<MissileLaunch>,
        player:         &Player,
        action:         Action,
    )
        -> Option<()>
    {
        let craft = crafts.get_mut(&self.craft)
            .or_else(|| {
                warn!("Craft not found: {:?}", self.craft);
                None
            })?;
        let body = bodies.get(&craft.body)
            .or_else(|| {
                warn!("Body not found: {:?}", craft.body);
                None
            })?;

        if craft.owner != player.id {
            return None;
        }

        match action.kind {
            input::EventKind::Rotate(rotation) => {
                self.rotation = rotation;
            }
            input::EventKind::Thrust(thrust) => {
                craft.engine_on = thrust;
            }
            input::EventKind::LaunchMissile { target } => {
                let missile = self.launch_missile(
                    craft.owner,
                    &body,
                    target,
                );
                if let Some(missile) = missile {
                    missile_launch.push(MissileLaunch { missile });
                }
            }
        }

        Some(())
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

    pub fn update(&self,
        bodies: &mut Store<Body>,
        crafts: &Store<Craft>,
    )
        -> Option<()>
    {
        let     craft = crafts.get(&self.craft)?;
        let mut body  = bodies.get_mut(&craft.body)?;

        let rotation = self.rotation as i32 as f32;
        body.rot = Rad::full_turn() * 0.6 * rotation;

        Some(())
    }

    pub fn remove(
        handle:     StrongHandle<Ship>,
        bodies:     &mut Store<Body>,
        crafts:     &mut Store<Craft>,
        directions: &mut Store<Direction>,
        fuels:      &mut Store<Fuel>,
        healths:    &mut Store<Health>,
        positions:  &mut Store<Position>,
        ships:      &mut Store<Ship>,
        velocities: &mut Store<Velocity>,
    )
        -> Option<()>
    {
        let ship = ships.remove(handle)?;
        Craft::remove(
            ship.craft,
            bodies,
            crafts,
            directions,
            fuels,
            healths,
            positions,
            velocities,
        )
    }
}
