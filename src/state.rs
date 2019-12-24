pub mod body;
pub mod engine;
pub mod missile;
pub mod ship;


pub use self::{
    body::Body,
    engine::Engine,
    missile::Missile,
    ship::Ship,
};


use hecs::World;

use crate::input::Event;


pub const WORLD_SIZE: f32 = 1000.0;


pub struct State {
    pub world: World,
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();

        world.spawn((Ship::new(), Engine::new(), Body::new()));

        Self {
            world,
        }
    }

    pub fn handle_input(&mut self, event: Event) {
        match event {
            Event::Rotate(rotation) => {
                for (_, (ship,)) in &mut self.world.query::<(&mut Ship,)>() {
                    ship.rotation = rotation;
                }
            }
            Event::Thrust(thrust) => {
                let query = &mut self.world.query::<(&Ship, &mut Engine)>();
                for (_, (_, engine)) in query {
                    engine.enabled = thrust;
                }
            }
            Event::LaunchMissile => {
                let mut missiles = Vec::new();
                {
                    let query = &mut self.world.query::<(&Ship, &Body)>();
                    for (_, (ship, body)) in query {
                        let missile = ship.launch_missile(body);
                        missiles.push(missile);
                    }
                }

                for missile in missiles {
                    self.world.spawn(missile);
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_ships();
        self.update_engines();
        self.update_bodies(dt);
    }

    fn update_ships(&mut self) {
        let query = &mut self.world.query::<(&mut Ship, &mut Body)>();

        for (_, (ship, body)) in query {
            ship.update(body);
        }
    }

    fn update_engines(&mut self) {
        let query = &mut self.world.query::<(&Engine, &mut Body)>();

        for (_, (engine, body)) in query {
            engine.update(body)
        }
    }

    fn update_bodies(&mut self, dt: f32) {
        for (_, (body,)) in &mut self.world.query::<(&mut Body,)>() {
            body.update(dt);
            body.enforce_boundary(WORLD_SIZE);
        }
    }
}
