pub mod components;
pub mod entities;
pub mod events;
pub mod systems;


use std::net::SocketAddr;

use hecs::Entity;

use crate::{
    input,
    world::{
        DeSpawned,
        World,
    },
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

    de_spawned: DeSpawned,
}

impl State {
    pub fn new() -> Self {
        Self {
            world:      World::new(),
            events:     Events::new(),
            de_spawned: DeSpawned::new(),
        }
    }

    pub fn push(&mut self) -> events::Push {
        self.events.push()
    }

    pub fn handle_input(&mut self, player: SocketAddr, event: input::Event) {
        match event {
            input::Event::Rotate(rotation) => {
                systems::input::handle_rotate(
                    self.world.query(),
                    player,
                    rotation,
                );
            }
            input::Event::Thrust(thrust) => {
                systems::input::handle_thrust(
                    self.world.query(),
                    player,
                    thrust,
                );
            }
            input::Event::LaunchMissile => {
                systems::input::handle_launch(
                    self.world.query(),
                    player,
                    &mut self.events.push(),
                );
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        systems::update::update_ships(self.world.query());
        systems::update::update_engines(self.world.query(), dt);
        systems::update::update_bodies(self.world.query(), WORLD_SIZE, dt);
        systems::update::update_missiles(
            self.world.query(),
            &mut self.events.push(),
        );
        systems::update::update_explosions(
            self.world.query(),
            dt,
            &mut self.events.push(),
        );
    }

    pub fn dispatch(&mut self) {
        let mut world = self.world.spawn(&mut self.de_spawned);

        for event in self.events.drain() {
            match event {
                Event::ConnectPlayer { player } => {
                    world.spawn(entities::ship(player));
                }
                Event::LaunchMissile { missile } => {
                    world.spawn(missile);
                }
                Event::ExplodeMissile { missile, explosion } => {
                    world.despawn(missile)
                        .expect("Missile should exist");
                    world.spawn(explosion);
                }
                Event::RemoveExplosion { explosion } => {
                    world.despawn(explosion)
                        .expect("Explosion should exist");
                }
            }
        }
    }

    pub fn spawned(&mut self) -> impl Iterator<Item=Entity> + '_ {
        self.de_spawned.spawned.drain(..)
    }

    pub fn despawned(&mut self) -> impl Iterator<Item=Entity> + '_ {
        self.de_spawned.despawned.drain(..)
    }
}
