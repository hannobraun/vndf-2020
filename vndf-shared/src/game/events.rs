use std::{
    collections::VecDeque,
    net::SocketAddr,
};

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

    pub fn push(&mut self) -> Push {
        Push(&mut self.0)
    }

    pub fn drain(&mut self) -> impl Iterator<Item=Event> + '_ {
        self.0.drain(..)
    }
}


pub struct Push<'r>(&'r mut VecDeque<Event>);

impl Push<'_> {
    pub fn connect_player(&mut self, player: SocketAddr) {
        self.0.push_back(Event::ConnectPlayer { player });
    }

    pub fn launch_missile(&mut self, missile: Missile) {
        self.0.push_back(Event::LaunchMissile(missile));
    }

    pub fn explode_missile(&mut self, missile: Entity, explosion: Explosion) {
        self.0.push_back(Event::ExplodeMissile { missile, explosion });
    }

    pub fn remove_explosion(&mut self, explosion: Entity) {
        self.0.push_back(Event::RemoveExplosion(explosion))
    }
}


pub enum Event {
    ConnectPlayer {
        player: SocketAddr,
    },
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
            Self::ConnectPlayer { player } => {
                world.spawn(entities::ship(player));
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
