use std::collections::VecDeque;

use hecs::Entity;

use crate::game::entities::{
    Explosion,
    Missile,
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
