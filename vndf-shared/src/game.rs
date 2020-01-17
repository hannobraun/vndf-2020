pub mod components;
pub mod entities;
pub mod indices;
pub mod in_event;
pub mod systems;


use std::net::SocketAddr;

use hecs::Entity;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    events::{
        Events,
        Push,
    },
    world::World,
};

use self::{
    components::Ship,
    indices::Indices,
    in_event::InEvent,
};


pub const WORLD_SIZE: f32 = 1000.0;

pub const TARGET_FPS: u32 = 60;
pub const FRAME_TIME: f32 = 1.0 / TARGET_FPS as f32;


pub struct State {
    world:     World,
    in_events: Events<InEvent>,
    despawned: Vec<Entity>,
    indices:   Indices,
    next_id:   PlayerId,
}

impl State {
    pub fn new() -> Self {
        Self {
            world:     World::new(),
            in_events: Events::new(),
            despawned: Vec::new(),
            indices:   Indices::new(),
            next_id:   PlayerId::first(),
        }
    }

    pub fn push(&mut self) -> Push<InEvent> {
        self.in_events.push()
    }

    pub fn dispatch(&mut self) {
        let despawned = &mut self.despawned;
        let mut on_despawn = |entity| despawned.push(entity);

        while let Some(event) = self.in_events.next() {
            match event {
                InEvent::Update { dt } => {
                    systems::players::update_ships(
                        self.world.query(),
                    );
                    systems::crafts::update_crafts(
                        self.world.query(),
                        dt,
                    );
                    systems::crafts::update_bodies(
                        self.world.query(),
                        WORLD_SIZE,
                        dt,
                    );
                    systems::missiles::update_missiles(
                        self.world.query(),
                        &mut self.in_events.push(),
                    );
                    systems::missiles::update_explosions(
                        self.world.query(),
                        dt,
                        &mut self.in_events.push(),
                    );
                }
                InEvent::ConnectPlayer { player } => {
                    let id = self.next_id.increment();

                    systems::players::connect_player(
                        &mut self.world.spawn(&mut on_despawn),
                        &mut self.indices,
                        id,
                        player,
                    );
                }
                InEvent::DisconnectPlayer { player } => {
                    systems::players::disconnect_player(
                        &mut self.world.spawn(&mut on_despawn),
                        &mut self.indices,
                        player,
                    );
                }
                InEvent::PlayerInput { player, event } => {
                    systems::players::handle_input(
                        self.world.query(),
                        &mut self.in_events.push(),
                        player,
                        event,
                    );
                }
                InEvent::LaunchMissile { missile } => {
                    systems::missiles::launch_missile(
                        &mut self.world.spawn(&mut on_despawn),
                        missile,
                    );
                }
                InEvent::ExplodeMissile { missile, explosion } => {
                    systems::missiles::explode_missile(
                        &mut self.world.spawn(&mut on_despawn),
                        missile,
                        explosion,
                    );
                }
                InEvent::RemoveExplosion { explosion } => {
                    systems::missiles::remove_explosion(
                        &mut self.world.spawn(&mut on_despawn),
                        explosion,
                    );
                }
            }
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn players(&self) -> Vec<SocketAddr> {
        self.world
            .inner()
            .query::<(&Ship,)>()
            .into_iter()
            .map(|(_, (ship,))| ship.player)
            .collect()
    }

    pub fn despawned(&mut self) -> impl Iterator<Item=Entity> + '_ {
        self.despawned.drain(..)
    }
}


#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct PlayerId(u64);

impl PlayerId {
    fn first() -> Self {
        Self(0)
    }

    fn increment(&mut self) -> Self {
        let current = self.0;
        self.0 += 1;
        Self(current)
    }
}
