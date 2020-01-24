pub mod entities;
pub mod features;
pub mod indices;


use std::net::SocketAddr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    cgs::{
        Handle,
        Store,
    },
    events,
    world::World,
};

use self::{
    features::{
        basics::{
            EntityRemoved,
            ItemRemoved,
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
            PlayerId,
            components::Player,
            events::{
                PlayerConnected,
                PlayerCreated,
                PlayerDisconnected,
                PlayerInput,
            },
        },
        ships::{
            self,
            components::Ship,
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
    ships:   Store<Ship>,

    death:               events::Buf<Death>,
    entity_removed:      events::Buf<EntityRemoved>,
    explosion_faded:     events::Buf<ExplosionFaded>,
    explosion_imminent:  events::Buf<ExplosionImminent>,
    item_removed:        events::Buf<ItemRemoved>,
    missile_launch:      events::Buf<MissileLaunch>,
    player_connected:    events::Buf<PlayerConnected>,
    player_created:      events::Buf<PlayerCreated>,
    player_disconnected: events::Buf<PlayerDisconnected>,
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
            ships:   Store::new(),

            death:               events::Buf::new(),
            entity_removed:      events::Buf::new(),
            explosion_faded:     events::Buf::new(),
            explosion_imminent:  events::Buf::new(),
            item_removed:        events::Buf::new(),
            missile_launch:      events::Buf::new(),
            player_connected:    events::Buf::new(),
            player_created:      events::Buf::new(),
            player_disconnected: events::Buf::new(),
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
        // Let's garbage-collect all items that need to be removed. This should
        // be an automatic process, probably controlled by reference-counting of
        // handles, but this will have to do for now.
        let mut remove = Vec::new();
        for (handle, ship) in &self.ships {
            let entity = hecs::Entity::from_bits(ship.entity);
            if !self.world.query().contains(entity) {
                remove.push(handle);
                let handle = ItemHandle::Ship(handle);
                self.item_removed.sink().push(ItemRemoved { handle });
            }
        }
        for handle in remove {
            self.ships.remove(handle);
        }

        let mut despawned = Vec::new();

        for Update { dt } in self.update.source().ready() {
            ships::systems::update_ships(
                &mut self.ships,
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
                &mut self.ships,
                &mut self.player_created.sink(),
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
                &mut self.ships,
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
                &self.ships,
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

    pub fn item_update(&mut self) -> impl Iterator<Item=(Handle, Item)> + '_ {
        self.ships.iter()
            .map(|(handle, item)| (handle, Item::Ship(*item)))
    }

    pub fn item_removed(&mut self) -> events::Source<ItemRemoved> {
        self.item_removed.source()
    }

    pub fn entity_removed(&mut self) -> events::Source<EntityRemoved> {
        self.entity_removed.source()
    }

    pub fn new_player(&mut self) -> events::Source<PlayerCreated> {
        self.player_created.source()
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum ItemHandle {
    Ship(Handle),
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Item {
    Ship(Ship),
}
