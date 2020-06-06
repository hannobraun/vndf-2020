use log::warn;
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
    world::{
        crafts::Craft,
        math::{
            Angle,
            Scalar,
        },
        physics::Body,
        players::Player,
    },
};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Ship {
    pub craft:    Handle<Craft>,
    pub rotation: Rotation,
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
            color,
        }
    }

    pub fn to_weak(&self) -> Self {
        Self {
            craft:    self.craft.as_weak(),
            rotation: self.rotation.clone(),
            color:    self.color.clone(),
        }
    }

    pub fn apply_input(&mut self,
        crafts: &mut store::Strong<Craft>,
        player: &Player,
        action: Action,
    )
        -> Option<()>
    {
        let craft = crafts.get_mut(&self.craft)
            .or_else(|| {
                warn!("Craft not found: {:?}", self.craft);
                None
            })?;

        if craft.owner != player.id {
            return None;
        }

        match action.kind {
            action::Kind::Rotate(rotation) => {
                self.rotation = rotation;
            }
            action::Kind::Thrust(thrust) => {
                craft.engine_on = thrust;
            }
        }

        Some(())
    }

    pub fn update(&self,
        bodies: &mut store::Strong<Body>,
        crafts: &store::Strong<Craft>,
    )
        -> Option<()>
    {
        let     craft = crafts.get(&self.craft)?;
        let mut body  = bodies.get_mut(&craft.body)?;

        let rotation = self.rotation as i32 as Scalar;
        body.rot = Angle::two_pi() * 0.6 * rotation;

        Some(())
    }
}
