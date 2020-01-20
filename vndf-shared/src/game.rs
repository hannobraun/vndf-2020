pub mod components;
pub mod entities;
pub mod features;
pub mod indices;
pub mod in_event;
pub mod systems;


use std::net::SocketAddr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    events::{
        self,
        Events,
        Push,
    },
    world::World,
};

use self::{
    components::Player,
    features::{
        basics::{
            EntityRemoved,
            Update,
        },
        explosions::{
            self,
            events::ExplosionImminent,
        },
        health::{
            self,
            events::Death,
        },
        missiles::MissileLaunch,
        players::{
            PlayerConnected,
            PlayerDisconnected,
            PlayerEntityCreated,
            PlayerInput,
        },
    },
    indices::Indices,
    in_event::InEvent,
};


pub const WORLD_SIZE: f32 = 1000.0;

pub const TARGET_FPS: u32 = 60;
pub const FRAME_TIME: f32 = 1.0 / TARGET_FPS as f32;


pub struct State {
    world:     World,
    in_events: Events<InEvent>,
    indices:   Indices,
    next_id:   PlayerId,

    death:                 events::Buf<Death>,
    entity_removed:        events::Buf<EntityRemoved>,
    explosion_imminent:    events::Buf<ExplosionImminent>,
    missile_launch:        events::Buf<MissileLaunch>,
    player_connected:      events::Buf<PlayerConnected>,
    player_disconnected:   events::Buf<PlayerDisconnected>,
    player_entity_created: events::Buf<PlayerEntityCreated>,
    player_input:          events::Buf<PlayerInput>,
    update:                events::Buf<Update>,
}

impl State {
    pub fn new() -> Self {
        Self {
            world:     World::new(),
            in_events: Events::new(),
            indices:   Indices::new(),
            next_id:   PlayerId::first(),

            death:                 events::Buf::new(),
            entity_removed:        events::Buf::new(),
            explosion_imminent:    events::Buf::new(),
            missile_launch:        events::Buf::new(),
            player_connected:      events::Buf::new(),
            player_disconnected:   events::Buf::new(),
            player_entity_created: events::Buf::new(),
            player_input:          events::Buf::new(),
            update:                events::Buf::new(),
        }
    }

    pub fn push(&mut self) -> Push<InEvent> {
        self.in_events.push()
    }

    pub fn player_connected(&mut self) -> events::Sink<PlayerConnected> {
        self.player_connected.sink()
    }

    pub fn player_disconnected(&mut self) -> events::Sink<PlayerDisconnected> {
        self.player_disconnected.sink()
    }

    pub fn player_input(&mut self) -> events::Sink<PlayerInput> {
        self.player_input.sink()
    }

    pub fn update(&mut self) -> events::Sink<Update> {
        self.update.sink()
    }

    pub fn dispatch(&mut self) {
        let mut despawned = Vec::new();

        for Update { dt } in self.update.source().ready() {
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
            );
            systems::explosions::update_explosions(
                self.world.query(),
                dt,
                &mut self.in_events.push(),
            );
            health::systems::check_health(
                self.world.query(),
                &mut self.death.sink(),
            );
        }
        for event in self.player_connected.source().ready() {
            let PlayerConnected { addr, color } = event;

            let id = self.next_id.increment();

            systems::players::connect_player(
                &mut self.world.spawn(&mut despawned),
                &mut self.player_entity_created.sink(),
                &mut self.indices,
                id,
                addr,
                color,
            );
        }
        for event in self.player_disconnected.source().ready() {
            let PlayerDisconnected { addr } = event;

            systems::players::disconnect_player(
                &mut self.world.spawn(&mut despawned),
                &mut self.indices,
                addr,
            );
        }
        for PlayerInput { addr, event } in self.player_input.source().ready() {
            systems::players::handle_input(
                self.world.query(),
                &mut self.missile_launch.sink(),
                &mut self.indices,
                addr,
                event,
            );
        }
        for MissileLaunch { missile } in self.missile_launch.source().ready() {
            systems::missiles::launch_missile(
                &mut self.world.spawn(&mut despawned),
                missile,
            );
        }
        for Death { entity } in self.death.source().ready() {
            let explosion = explosions::systems::explode_entity(
                self.world.query(),
                entity,
            );
            health::systems::remove_entity(
                &mut self.world.spawn(&mut despawned),
                entity,
            );
            if let Some(explosion) = explosion {
                explosions::systems::create_explosion(
                    &mut self.world.spawn(&mut despawned),
                    &mut self.explosion_imminent.sink(),
                    explosion,
                );
            }
        }
        for event in self.explosion_imminent.source().ready() {
            let ExplosionImminent { explosion } = event;

            systems::explosions::damage_nearby(
                &mut self.world.query(),
                explosion,
            );
        }

        while let Some(event) = self.in_events.next() {
            match event {
                InEvent::RemoveExplosion { explosion } => {
                    systems::explosions::remove_explosion(
                        &mut self.world.spawn(&mut despawned),
                        explosion,
                    );
                }
            }
        }

        for entity in despawned {
            self.entity_removed.sink().push(EntityRemoved { entity });
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn players(&self) -> Vec<SocketAddr> {
        self.world
            .inner()
            .query::<(&Player,)>()
            .into_iter()
            .map(|(_, (player,))| player.addr)
            .collect()
    }

    pub fn entity_removed(&mut self) -> events::Source<EntityRemoved> {
        self.entity_removed.source()
    }

    pub fn new_player(&mut self) -> events::Source<PlayerEntityCreated> {
        self.player_entity_created.source()
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
