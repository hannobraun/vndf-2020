use std::{
    collections::VecDeque,
    net::SocketAddr,
};

use hecs::Entity;

use crate::{
    game::entities::{
        Explosion,
        Missile,
    },
    input,
};


pub struct Events(VecDeque<Event>);

impl Events {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self) -> Push {
        Push(&mut self.0)
    }

    pub fn next(&mut self) -> Option<Event> {
        self.0.pop_front()
    }
}


pub struct Push<'r>(&'r mut VecDeque<Event>);

impl Push<'_> {
    pub fn connect_player(&mut self, player: SocketAddr) {
        self.0.push_back(Event::ConnectPlayer { player });
    }

    pub fn player_input(&mut self, player: SocketAddr, event: input::Event) {
        self.0.push_back(Event::PlayerInput { player, event });
    }

    pub fn launch_missile(&mut self, missile: Missile) {
        self.0.push_back(Event::LaunchMissile { missile });
    }

    pub fn explode_missile(&mut self, missile: Entity, explosion: Explosion) {
        self.0.push_back(Event::ExplodeMissile { missile, explosion });
    }

    pub fn remove_explosion(&mut self, explosion: Entity) {
        self.0.push_back(Event::RemoveExplosion { explosion })
    }
}


pub enum Event {
    ConnectPlayer {
        player: SocketAddr,
    },
    PlayerInput {
        player: SocketAddr,
        event:  input::Event,
    },
    LaunchMissile {
        missile: Missile,
    },
    ExplodeMissile {
        missile:   Entity,
        explosion: Explosion,
    },
    RemoveExplosion {
        explosion: Entity,
    },
}
