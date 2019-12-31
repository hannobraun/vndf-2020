pub mod components;
pub mod entities;
pub mod events;
pub mod systems;


use hecs::World;

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
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();

        world.spawn(entities::ship());

        Self {
            world,
            events: Events::new(),
        }
    }

    pub fn handle_input(&mut self, event: input::Event) {
        let mut world = world::Query::new(&mut self.world);

        match event {
            input::Event::Rotate(rotation) => {
                systems::input::handle_rotate(&mut world, rotation);
            }
            input::Event::Thrust(thrust) => {
                systems::input::handle_thrust(&mut world, thrust);
            }
            input::Event::LaunchMissile => {
                systems::input::handle_launch(&mut world, &mut self.events);
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut world = world::Query::new(&mut self.world);

        systems::update::update_ships(&mut world);
        systems::update::update_engines(&mut world, dt);
        systems::update::update_bodies(&mut world, WORLD_SIZE, dt);
        systems::update::update_missiles(&mut world, &mut self.events);
        systems::update::update_explosions(&mut world, dt);

        for event in self.events.drain() {
            match event {
                Event::LaunchMissile(missile) => {
                    self.world.spawn(missile);
                }
                Event::ExplodeMissile { missile, explosion } => {
                    self.world.despawn(missile)
                        .expect("Missile should exist");
                    self.world.spawn(explosion);
                }
            }
        }
    }
}
