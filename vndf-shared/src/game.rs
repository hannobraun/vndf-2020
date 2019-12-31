pub mod components;
pub mod entities;
pub mod systems;

use crate::{
    input::Event,
    world,
};


pub const WORLD_SIZE: f32 = 1000.0;

pub const TARGET_FPS: u32 = 60;
pub const FRAME_TIME: f32 = 1.0 / TARGET_FPS as f32;


pub struct State {
    pub world: hecs::World,
}

impl State {
    pub fn new() -> Self {
        let mut world = hecs::World::new();

        world.spawn(entities::ship());

        Self {
            world,
        }
    }

    pub fn handle_input(&mut self, event: Event) {
        let mut world = world::Query::new(&mut self.world);

        match event {
            Event::Rotate(rotation) => {
                systems::input::handle_rotate(&mut world, rotation);
            }
            Event::Thrust(thrust) => {
                systems::input::handle_thrust(&mut world, thrust);
            }
            Event::LaunchMissile => {
                systems::input::handle_launch(&mut world);
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut world = world::Query::new(&mut self.world);

        systems::update::update_ships(&mut world);
        systems::update::update_engines(&mut world, dt);
        systems::update::update_bodies(&mut world, WORLD_SIZE, dt);
        systems::update::update_missiles(&mut world);
        systems::update::update_explosions(&mut world, dt);
    }
}
