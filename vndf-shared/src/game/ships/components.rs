use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    cgs::{
        Handle,
        Store,
    },
    events,
    game::{
        crafts::Craft,
        health::Health,
        missiles::{
            MissileEntity,
            MissileLaunch,
        },
        physics::{
            Body,
            Position,
        },
        players::{
            Player,
            PlayerId,
        },
    },
    input::{
        self,
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
    pub craft:    Handle,
    pub rotation: Rotation,
    pub missiles: u64,
    pub color:    [f32; 3],
}

impl Ship {
    pub fn new(
        craft:  Handle,
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
        event:          input::Event,
    )
        -> Option<()>
    {
        let craft = crafts.get_mut(self.craft)?;
        let body  = bodies.get(craft.body)?;

        if craft.owner != player.id {
            return None;
        }

        match event {
            input::Event::Rotate(rotation) => {
                self.rotation = rotation;
            }
            input::Event::Thrust(thrust) => {
                craft.engine_on = thrust;
            }
            input::Event::LaunchMissile { target } => {
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
        let     craft = crafts.get(self.craft)?;
        let mut body  = bodies.get_mut(craft.body)?;

        let rotation = self.rotation as i32 as f32;
        body.rot = Rad::full_turn() * 0.6 * rotation;

        Some(())
    }

    pub fn remove(
        handle:    Handle,
        bodies:    &mut Store<Body>,
        crafts:    &mut Store<Craft>,
        healths:   &mut Store<Health>,
        positions: &mut Store<Position>,
        ships:     &mut Store<Ship>,
    )
        -> Option<()>
    {
        let ship = ships.remove(handle)?;
        Craft::remove(ship.craft, bodies, crafts, healths, positions)
    }
}
