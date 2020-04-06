use log::warn;
use rinnsal::EventSink;
use serde::{
    Deserialize,
    Serialize,
};
use toadster::{
    Handle,
    store,
};

use crate::{
    action::{
        self,
        Action,
        Rotation,
    },
    game::{
        crafts::Craft,
        missiles::{
            MissileEntity,
            MissileLaunch,
        },
        physics::Body,
        players::{
            Player,
            PlayerId,
        },
    },
    math::{
        prelude::*,
        Pnt2,
        Rad,
    },
};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Ship {
    pub craft:    Handle<Craft>,
    pub rotation: Rotation,
    pub missiles: u64,
    pub color:    [f32; 3],
}

impl Ship {
    pub fn new(
        craft: impl Into<Handle<Craft>>,
        color: [f32; 3],
    )
        -> Self
    {
        Self {
            craft:    craft.into(),
            rotation: Rotation::None,
            missiles: 16,
            color,
        }
    }

    pub fn to_weak(&self) -> Self {
        Self {
            craft:    self.craft.as_weak(),
            rotation: self.rotation.clone(),
            missiles: self.missiles.clone(),
            color:    self.color.clone(),
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
            action::EventKind::Rotate(rotation) => {
                self.rotation = rotation;
            }
            action::EventKind::Thrust(thrust) => {
                craft.engine_on = thrust;
            }
            action::EventKind::LaunchMissile { target } => {
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
}
