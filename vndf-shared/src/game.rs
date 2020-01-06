pub mod components;
pub mod entities;
pub mod events;
pub mod systems;


use std::net::SocketAddr;

use hecs::{
    Entity,
    World,
};

use crate::{
    input,
    world,
};

use self::events::{
    Event,
    Events,
};


pub const WORLD_SIZE: f32 = 1000.0;

pub const TARGET_FPS: u32 = 60;
pub const FRAME_TIME: f32 = 1.0 / TARGET_FPS as f32;


pub struct State {
    pub world:  World,
    pub events: Events,

    spawned:   Vec<Entity>,
    despawned: Vec<Entity>,
}

impl State {
    pub fn new() -> Self {
        Self {
            world:     World::new(),
            events:    Events::new(),
            spawned:   Vec::new(),
            despawned: Vec::new(),
        }
    }

    pub fn connect_player(&mut self, player: SocketAddr) {
        self.events.push(Event::SpawnShip { player });
    }

    pub fn handle_input(&mut self, player: SocketAddr, event: input::Event) {
        let mut world = world::Query {
            world: &mut self.world,
        };

        match event {
            input::Event::Rotate(rotation) => {
                systems::input::handle_rotate(
                    &mut world,
                    player,
                    rotation,
                );
            }
            input::Event::Thrust(thrust) => {
                systems::input::handle_thrust(
                    &mut world,
                    player,
                    thrust,
                );
            }
            input::Event::LaunchMissile => {
                systems::input::handle_launch(
                    &mut world,
                    player,
                    &mut self.events,
                );
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut world = world::Query {
            world: &mut self.world,
        };

        systems::update::update_ships(&mut world);
        systems::update::update_engines(&mut world, dt);
        systems::update::update_bodies(&mut world, WORLD_SIZE, dt);
        systems::update::update_missiles(&mut world, &mut self.events);
        systems::update::update_explosions(&mut world, dt, &mut self.events);

        let mut world = world::Spawn {
            world:     &mut self.world,
            spawned:   &mut self.spawned,
            despawned: &mut self.despawned,
        };

        for event in self.events.drain() {
            event.handle(&mut world);
        }
    }

    pub fn spawned(&mut self) -> impl Iterator<Item=Entity> + '_ {
        self.spawned.drain(..)
    }

    pub fn despawned(&mut self) -> impl Iterator<Item=Entity> + '_ {
        self.despawned.drain(..)
    }
}
