use bach::EventSink;
use log::warn;
use serde::{
    Deserialize,
    Serialize,
};
use toadster::{
    handle,
    store,
};

use crate::{
    game::{
        crafts::{
            Craft,
            Fuel,
        },
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


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Ship {
    pub craft:    handle::Strong<Craft>,
    pub rotation: Rotation,
    pub missiles: u64,
    pub color:    [f32; 3],
}

impl Ship {
    pub fn new(
        craft:  handle::Strong<Craft>,
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
        bodies:         &store::Strong<Body>,
        crafts:         &mut store::Strong<Craft>,
        missile_launch: &mut EventSink<MissileLaunch>,
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
            Some(MissileEntity { owner, origin: body.clone(), target })
        }
        else {
            None
        }
    }

    pub fn update(&self,
        bodies: &mut store::Strong<Body>,
        crafts: &store::Strong<Craft>,
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
        handle:     handle::Strong<Ship>,
        bodies:     &mut store::Strong<Body>,
        crafts:     &mut store::Strong<Craft>,
        directions: &mut store::Strong<Direction>,
        fuels:      &mut store::Strong<Fuel>,
        positions:  &mut store::Strong<Position>,
        ships:      &mut store::Strong<Ship>,
        velocities: &mut store::Strong<Velocity>,
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
            positions,
            velocities,
        )
    }
}
