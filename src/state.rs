pub mod body;
pub mod missile;
pub mod ship;


pub use self::{
    body::Body,
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

        world.spawn((Body::new(), Ship::new()));
        world.spawn((Body::new(), Missile::new()));

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
                for (_, (ship,)) in &mut self.world.query::<(&mut Ship,)>() {
                    ship.thrust = thrust;
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

    pub fn update(&mut self, frame_time: f32) {
        self.update_ships();
        self.update_missiles();
        self.update_bodies(frame_time);
    }

    fn update_ships(&mut self) {
        let query = &mut self.world.query::<(&mut Ship, &mut Body)>();

        for (_, (player, body)) in query {
            player.apply_input(body);
        }
    }

    fn update_missiles(&mut self) {
        let query = &mut self.world.query::<(&mut Missile, &mut Body)>();

        for (_, (missile, body)) in query {
            missile.update(body);
        }
    }

    fn update_bodies(&mut self, frame_time: f32) {
        for (_, (body,)) in &mut self.world.query::<(&mut Body,)>() {
            body.update(frame_time);
            body.enforce_boundary(WORLD_SIZE);
        }
    }
}
