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
    input::{
        self,
        Rotation,
    },
    math::{
        prelude::*,
        Pnt2,
        Rad,
    },
    world,
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Ship {
    pub entity: u64,
    pub craft:  Handle,

    pub rotation: Rotation,
    pub missiles: u64,
    pub color:    [f32; 3],
}

impl Ship {
    pub fn new(entity: hecs::Entity, craft: Handle, color: [f32; 3]) -> Self {
        Self {
            entity:   entity.to_bits(),
            craft,
            rotation: Rotation::None,
            missiles: 16,
            color,
        }
    }

    pub fn apply_input(&mut self,
        world:          &world::Query,
        crafts:         &mut Store<Craft>,
        missile_launch: &mut events::Sink<MissileLaunch>,
        player:         &Player,
        event:          input::Event,
    )
        -> Option<()>
    {
        let entity = hecs::Entity::from_bits(self.entity);

        let body  = world.get::<Body>(entity).ok()?;
        let craft = crafts.get_mut(self.craft)?;

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

    pub fn update(&self, body: &mut Body) {
        let rotation = self.rotation as i32 as f32;
        body.rot = Rad::full_turn() * 0.6 * rotation;
    }
}
