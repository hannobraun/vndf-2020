pub mod components;
pub mod entities;
pub mod events;
pub mod systems;


use hecs::Entity;

use crate::world::{
    DeSpawned,
    World,
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

    pub fn update(&mut self, dt: f32) {
        systems::ships::update_ships(self.world.query());
        systems::crafts::update_engines(self.world.query(), dt);
        systems::crafts::update_bodies(self.world.query(), WORLD_SIZE, dt);
        systems::missiles::update_missiles(
            self.world.query(),
            &mut self.events.push(),
        );
        systems::missiles::update_explosions(
            self.world.query(),
            dt,
            &mut self.events.push(),
        );
    }

    pub fn dispatch(&mut self) {
        while let Some(event) = self.events.next() {
            match event {
                Event::ConnectPlayer { player } => {
                    systems::ships::create_ship(
                        &mut self.world.spawn(&mut self.de_spawned),
                        player,
                    );
                }
                Event::PlayerInput { player, event } => {
                    systems::ships::handle_input(
                        self.world.query(),
                        &mut self.events.push(),
                        player,
                        event,
                    );
                }
                Event::LaunchMissile { missile } => {
                    systems::missiles::launch_missile(
                        &mut self.world.spawn(&mut self.de_spawned),
                        missile,
                    );
                }
                Event::ExplodeMissile { missile, explosion } => {
                    systems::missiles::explode_missile(
                        &mut self.world.spawn(&mut self.de_spawned),
                        missile,
                        explosion,
                    );
                }
                Event::RemoveExplosion { explosion } => {
                    systems::missiles::remove_explosion(
                        &mut self.world.spawn(&mut self.de_spawned),
                        explosion,
                    );
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
