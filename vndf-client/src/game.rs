pub mod components;
pub mod entities;
pub mod systems;


use hecs::World;

use crate::shared::input::Event;


pub const WORLD_SIZE: f32 = 1000.0;

pub const TARGET_FPS: u32 = 60;
pub const FRAME_TIME: f32 = 1.0 / TARGET_FPS as f32;


pub struct State {
    pub world: World,
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();

        world.spawn(entities::ship());

        Self {
            world,
        }
    }

    pub fn handle_input(&mut self, event: Event) {
        match event {
            Event::Rotate(rotation) => {
                systems::input::handle_rotate(&mut self.world, rotation);
            }
            Event::Thrust(thrust) => {
                systems::input::handle_thrust(&mut self.world, thrust);
            }
            Event::LaunchMissile => {
                systems::input::handle_launch(&mut self.world);
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        systems::update::update_ships(&mut self.world);
        systems::update::update_engines(&mut self.world, dt);
        systems::update::update_bodies(&mut self.world, WORLD_SIZE, dt);
        systems::update::update_missiles(&mut self.world);
        systems::update::update_explosions(&mut self.world, dt);
    }
}
