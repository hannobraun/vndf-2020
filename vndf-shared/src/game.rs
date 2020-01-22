pub mod entities;
pub mod features;
pub mod indices;


use std::net::SocketAddr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    cgs::Store,
    events,
    world::World,
};

use self::{
    features::{
        basics::{
            EntityRemoved,
            Update,
        },
        crafts,
        explosions::{
            self,
            events::{
                ExplosionFaded,
                ExplosionImminent,
            },
        },
        health::{
            self,
            events::Death,
        },
        missiles::{
            self,
            events::MissileLaunch,
        },
        players::{
            self,
            events::{
                PlayerConnected,
                PlayerDisconnected,
                PlayerItemCreated,
                PlayerInput,
            },
            items::Player,
        },
    },
    indices::Indices,
};


pub const WORLD_SIZE: f32 = 1000.0;

pub const TARGET_FPS: u32 = 60;
pub const FRAME_TIME: f32 = 1.0 / TARGET_FPS as f32;


pub struct State {
    world:   World,
    indices: Indices,
    next_id: PlayerId,

    players: Store<Player>,

    death:               events::Buf<Death>,
    entity_removed:      events::Buf<EntityRemoved>,
    explosion_faded:     events::Buf<ExplosionFaded>,
    explosion_imminent:  events::Buf<ExplosionImminent>,
    missile_launch:      events::Buf<MissileLaunch>,
    player_connected:    events::Buf<PlayerConnected>,
    player_disconnected: events::Buf<PlayerDisconnected>,
    player_item_created: events::Buf<PlayerItemCreated>,
    player_input:        events::Buf<PlayerInput>,
    update:              events::Buf<Update>,
}

impl State {
    pub fn new() -> Self {
        Self {
            world:   World::new(),
            indices: Indices::new(),
            next_id: PlayerId::first(),

            players: Store::new(),

            death:               events::Buf::new(),
            entity_removed:      events::Buf::new(),
            explosion_faded:     events::Buf::new(),
            explosion_imminent:  events::Buf::new(),
            missile_launch:      events::Buf::new(),
            player_connected:    events::Buf::new(),
            player_disconnected: events::Buf::new(),
            player_item_created: events::Buf::new(),
            player_input:        events::Buf::new(),
            update:              events::Buf::new(),
        }
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
            players::systems::update_ships(
                self.world.query(),
            );
            crafts::systems::update_crafts(
                self.world.query(),
                dt,
            );
            crafts::systems::update_bodies(
                self.world.query(),
                WORLD_SIZE,
                dt,
            );
            missiles::systems::update_missiles(
                self.world.query(),
            );
            explosions::systems::update_explosions(
                self.world.query(),
                dt,
                &mut self.explosion_faded.sink(),
            );
            health::systems::check_health(
                self.world.query(),
                &mut self.death.sink(),
            );
        }
        for event in self.player_connected.source().ready() {
            let PlayerConnected { addr, color } = event;

            let id = self.next_id.increment();

            players::systems::connect_player(
                &mut self.world.spawn(&mut despawned),
                &mut self.players,
                &mut self.player_item_created.sink(),
                &mut self.indices,
                id,
                addr,
                color,
            );
        }
        for event in self.player_disconnected.source().ready() {
            let PlayerDisconnected { addr } = event;

            players::systems::disconnect_player(
                &mut self.players,
                &mut self.indices,
                addr,
            );
        }
        for PlayerInput { addr, event } in self.player_input.source().ready() {
            players::systems::handle_input(
                self.world.query(),
                &self.players,
                &mut self.missile_launch.sink(),
                &mut self.indices,
                addr,
                event,
            );
        }
        for MissileLaunch { missile } in self.missile_launch.source().ready() {
            missiles::systems::launch_missile(
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

            explosions::systems::damage_nearby(
                &mut self.world.query(),
                explosion,
            );
        }
        for event in self.explosion_faded.source().ready() {
            let ExplosionFaded { entity } = event;

            explosions::systems::remove_explosion(
                &mut self.world.spawn(&mut despawned),
                entity,
            );
        }

        for handle in despawned {
            self.entity_removed.sink().push(EntityRemoved { handle });
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn players(&self) -> Vec<SocketAddr> {
        self.players
            .iter()
            .map(|(_, player)| player.addr)
            .collect()
    }

    pub fn entity_removed(&mut self) -> events::Source<EntityRemoved> {
        self.entity_removed.source()
    }

    pub fn new_player(&mut self) -> events::Source<PlayerItemCreated> {
        self.player_item_created.source()
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
