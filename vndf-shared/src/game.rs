pub mod base;
pub mod crafts;
pub mod explosions;
pub mod health;
pub mod missiles;
pub mod physics;
pub mod players;
pub mod ships;


use std::{
    collections::HashMap,
    net::SocketAddr,
};

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
};

use self::{
    base::{
        Component,
        ComponentRemoved,
        Update,
    },
    explosions::{
        Explosion,
        ExplosionFaded,
        ExplosionImminent,
    },
    health::{
        Death,
        Health,
    },
    missiles::{
        Missile,
        MissileLaunch,
    },
    players::{
        Player,
        PlayerConnected,
        PlayerCreated,
        PlayerDisconnected,
        PlayerId,
        PlayerInput,
    },
    ships::Ship,
};


pub const WORLD_SIZE: f32 = 1000.0;

pub const TARGET_FPS: u32 = 60;
pub const FRAME_TIME: f32 = 1.0 / TARGET_FPS as f32;


pub struct State {
    next_id: PlayerId,

    players_by_address: HashMap<SocketAddr, Handle>,

    physics: physics::Feature,
    crafts:  crafts::Feature,

    explosions: Store<Explosion>,
    healths:    Store<Health>,
    players:    Store<Player>,
    missiles:   Store<Missile>,
    ships:      Store<Ship>,

    death:               events::Buf<Death>,
    explosion_faded:     events::Buf<ExplosionFaded>,
    explosion_imminent:  events::Buf<ExplosionImminent>,
    component_removed:   events::Buf<ComponentRemoved>,
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
            next_id: PlayerId::first(),

            players_by_address: HashMap::new(),

            physics: physics::Feature::new(),
            crafts:  crafts::Feature::new(),

            explosions: Store::new(),
            healths:    Store::new(),
            missiles:   Store::new(),
            players:    Store::new(),
            ships:      Store::new(),

            death:               events::Buf::new(),
            explosion_faded:     events::Buf::new(),
            explosion_imminent:  events::Buf::new(),
            component_removed:   events::Buf::new(),
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
        for event in self.update.source().ready() {
            self.crafts.on_update(
                &event,
                &mut self.physics.bodies,
            );
            self.physics.on_update(
                &event,
                WORLD_SIZE,
            );

            let Update { dt } = event;

            ships::update_ships(
                &mut self.physics.bodies,
                &self.crafts.crafts,
                &mut self.ships,
            );
            missiles::update_targets(
                &mut self.physics.bodies,
                &self.crafts.crafts,
                &mut self.missiles,
            );
            missiles::update_guidances(
                &mut self.physics.bodies,
                &self.crafts.crafts,
                &mut self.missiles,
            );
            missiles::explode_missiles(
                &self.physics.bodies,
                &self.crafts.crafts,
                &mut self.healths,
                &self.missiles,
            );
            explosions::update_explosions(
                &mut self.explosions,
                dt,
                &mut self.explosion_faded.sink(),
            );
            health::check_health(
                &self.healths,
                &mut self.death.sink(),
            );
        }
        for event in self.player_connected.source().ready() {
            let PlayerConnected { addr, color } = event;

            let id = self.next_id.increment();

            players::connect_player(
                &mut self.physics.bodies,
                &mut self.crafts.crafts,
                &mut self.healths,
                &mut self.players,
                &mut self.ships,
                &mut self.player_created.sink(),
                &mut self.players_by_address,
                id,
                addr,
                color,
            );
        }
        for event in self.player_disconnected.source().ready() {
            let PlayerDisconnected { addr } = event;

            players::disconnect_player(
                &mut self.players,
                &mut self.players_by_address,
                addr,
            );
        }
        for PlayerInput { addr, event } in self.player_input.source().ready() {
            players::handle_input(
                &self.physics.bodies,
                &mut self.crafts.crafts,
                &self.players,
                &mut self.ships,
                &mut self.missile_launch.sink(),
                &mut self.players_by_address,
                addr,
                event,
            );
        }
        for MissileLaunch { missile } in self.missile_launch.source().ready() {
            missiles::launch_missile(
                &mut self.physics.bodies,
                &mut self.crafts.crafts,
                &mut self.healths,
                &mut self.missiles,
                missile,
            );
        }
        for Death { handle } in self.death.source().ready() {
            let explosion = explosions::explode_entity(
                &self.physics.bodies,
                &self.healths,
                handle,
            );
            health::remove_entity(
                handle,
                &mut self.physics.bodies,
                &mut self.crafts.crafts,
                &mut self.healths,
                &mut self.missiles,
                &mut self.ships,
            );
            if let Some(explosion) = explosion {
                explosions::create_explosion(
                    &mut self.physics.bodies,
                    &mut self.explosions,
                    &mut self.explosion_imminent.sink(),
                    explosion,
                );
            }
        }
        while let Some(event) = self.explosion_imminent.source().next() {
            let ExplosionImminent { handle } = event;

            explosions::damage_nearby(
                handle,
                &self.physics.bodies,
                &self.explosions,
                &mut self.healths,
            );
        }
        while let Some(event) = self.explosion_faded.source().next() {
            let ExplosionFaded { handle } = event;

            explosions::remove_explosion(
                &mut self.explosions,
                handle,
            );
        }
    }

    pub fn players(&self) -> Vec<SocketAddr> {
        self.players
            .iter()
            .map(|(_, player)| player.addr)
            .collect()
    }

    pub fn updates(&mut self)
        -> impl Iterator<Item=(Handle, Component)> + '_
    {
        let bodies = self.physics.bodies.iter()
            .map(|(handle, &c)| (handle, Component::Body(c)));
        let crafts = self.crafts.crafts.iter()
            .map(|(handle, &c)| (handle, Component::Craft(c)));
        let explosions = self.explosions.iter()
            .map(|(handle, &c)| (handle, Component::Explosion(c)));
        let healths = self.healths.iter()
            .map(|(handle, &c)| (handle, Component::Health(c)));
        let missiles = self.missiles.iter()
            .map(|(handle, &c)| (handle, Component::Missile(c)));
        let ships = self.ships.iter()
            .map(|(handle, &c)| (handle, Component::Ship(c)));

        bodies
            .chain(crafts)
            .chain(explosions)
            .chain(healths)
            .chain(missiles)
            .chain(ships)
    }

    pub fn component_removed(&mut self) -> events::Source<ComponentRemoved> {
        self.component_removed.source()
    }

    pub fn player_created(&mut self) -> events::Source<PlayerCreated> {
        self.player_created.source()
    }

    pub fn diagnostics(&self) -> Diagnostics {
        Diagnostics {
            num_bodies:     self.physics.bodies.len() as u64,
            num_crafts:     self.crafts.crafts.len()  as u64,
            num_explosions: self.explosions.len()     as u64,
            num_healths:    self.healths.len()        as u64,
            num_players:    self.players.len()        as u64,
            num_missiles:   self.missiles.len()       as u64,
            num_ships:      self.ships.len()          as u64,
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Diagnostics {
    pub num_bodies:     u64,
    pub num_crafts:     u64,
    pub num_explosions: u64,
    pub num_healths:    u64,
    pub num_players:    u64,
    pub num_missiles:   u64,
    pub num_ships:      u64,
}
