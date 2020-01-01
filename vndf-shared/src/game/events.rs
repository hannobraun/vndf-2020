use std::collections::VecDeque;

use hecs::Entity;

use crate::{
    game::entities::{
        self,
        Explosion,
        Missile,
    },
    world,
};


pub struct Events(VecDeque<Event>);

impl Events {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self, event: Event) {
        self.0.push_back(event);
    }

    pub fn drain(&mut self) -> impl Iterator<Item=Event> + '_ {
        self.0.drain(..)
    }
}


pub enum Event {
    SpawnShip,
    LaunchMissile(Missile),
    ExplodeMissile {
        missile:   Entity,
        explosion: Explosion,
    },
    RemoveExplosion(Entity),
}

impl Event {
    pub fn handle(self, world: &mut world::Spawn) {
        match self {
            Self::SpawnShip => {
                world.spawn(entities::ship());
            }
            Self::LaunchMissile(missile) => {
                world.spawn(missile);
            }
            Self::ExplodeMissile { missile, explosion } => {
                world.despawn(missile)
                    .expect("Missile should exist");
                world.spawn(explosion);
            }
            Self::RemoveExplosion(explosion) => {
                world.despawn(explosion)
                    .expect("Explosion should exist");
            }
        }
    }
}
