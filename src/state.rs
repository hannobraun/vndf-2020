pub mod body;
pub mod player;


pub use self::{
    body::Body,
    player::Player,
};


use hecs::World;

use crate::input::Input;


pub const WORLD_SIZE: f32 = 1000.0;


pub struct State {
    pub world: World,
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();

        world.spawn((Body::new(), Player::new()));

        Self {
            world,
        }
    }

    pub fn update(&mut self, frame_time: f32, input: Input) {
        self.update_players(input);
        self.update_bodies(frame_time);
    }

    fn update_players(&mut self, input: Input) {
        let query = &mut self.world.query::<(&mut Player, &mut Body)>();

        for (_, (player, body)) in query {
            player.input = input;
            player.apply_input(body);
        }
    }

    fn update_bodies(&mut self, frame_time: f32) {
        for (_, (body,)) in &mut self.world.query::<(&mut Body,)>() {
            body.update(frame_time);
            body.enforce_boundary();
        }
    }
}
