pub mod base;
pub mod crafts;
pub mod explosions;
pub mod health;
pub mod missiles;
pub mod physics;
pub mod players;
pub mod ships;


use std::net::SocketAddr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    cgs::Handle,
    events,
};

use self::{
    base::{
        Component,
        ComponentHandle,
        ComponentRemoved,
        Update,
    },
    players::{
        PlayerConnected,
        PlayerCreated,
        PlayerDisconnected,
        PlayerId,
        PlayerInput,
    },
};


pub const WORLD_SIZE: f32 = 1000.0;

pub const TARGET_FPS: u32 = 60;
pub const FRAME_TIME: f32 = 1.0 / TARGET_FPS as f32;


pub struct State {
    base:       base::Feature,
    crafts:     crafts::Feature,
    explosions: explosions::Feature,
    health:     health::Feature,
    missiles:   missiles::Feature,
    physics:    physics::Feature,
    players:    players::Feature,
    ships:      ships::Feature,
}

impl State {
    pub fn new() -> Self {
        Self {
            base:       base::Feature::new(),
            crafts:     crafts::Feature::new(),
            explosions: explosions::Feature::new(),
            health:     health::Feature::new(),
            missiles:   missiles::Feature::new(),
            physics:    physics::Feature::new(),
            players:    players::Feature::new(),
            ships:      ships::Feature::new(),
        }
    }

    pub fn player_connected(&mut self) -> events::Sink<PlayerConnected> {
        self.players.player_connected.sink()
    }

    pub fn player_disconnected(&mut self) -> events::Sink<PlayerDisconnected> {
        self.players.player_disconnected.sink()
    }

    pub fn player_input(&mut self) -> events::Sink<PlayerInput> {
        self.players.player_input.sink()
    }

    pub fn update(&mut self) -> events::Sink<Update> {
        self.base.update.sink()
    }

    pub fn dispatch(&mut self) {
        for event in self.base.update.source().ready() {
            self.crafts.on_update(
                &event,
                &mut self.physics.bodies,
            );
            self.explosions.on_update(
                &event,
            );
            self.physics.on_update(
                &event,
                WORLD_SIZE,
            );
            self.health.on_update();
            self.missiles.on_update(
                &mut self.physics.bodies,
                &self.crafts.crafts,
                &mut self.health.healths,
            );
            self.ships.on_update(
                &mut self.physics.bodies,
                &self.crafts.crafts,
            )
        }
        while let Some(event) = self.players.player_connected.source().next() {
            self.players.on_player_connected(
                &event,
                &mut self.physics.bodies,
                &mut self.crafts.crafts,
                &mut self.health.healths,
                &mut self.ships.ships,
            );
        }
        while let Some(event) =
            self.players.player_disconnected.source().next()
        {
            self.players.on_player_disconnected(&event);
        }
        while let Some(event) = self.players.player_input.source().next() {
            self.players.on_player_input(
                &event,
                &self.physics.bodies,
                &mut self.crafts.crafts,
                &mut self.ships.ships,
                &mut self.missiles.missile_launch.sink(),
            );
        }
        while let Some(event) = self.missiles.missile_launch.source().next() {
            self.missiles.on_missile_launch(
                event,
                &mut self.physics.bodies,
                &mut self.crafts.crafts,
                &mut self.health.healths,
            );
        }
        while let Some(event) = self.health.death.source().next() {
            self.explosions.on_death(
                &event,
                &mut self.physics.bodies,
                &self.health.healths,
            );
            self.health.on_death(
                &event,
                &mut self.physics.bodies,
                &mut self.crafts.crafts,
                &mut self.missiles.missiles,
                &mut self.ships.ships,
            );
        }
        while let Some(event) =
            self.explosions.explosion_imminent.source().next()
        {
            self.explosions.on_explosion_imminent(
                &event,
                &self.physics.bodies,
                &mut self.health.healths,
            )
        }
        while let Some(event) =
            self.explosions.explosion_faded.source().next()
        {
            self.explosions.on_explosion_faded(&event);
        }
    }

    pub fn players(&self) -> Vec<SocketAddr> {
        self.players.players
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
        let explosions = self.explosions.explosions.iter()
            .map(|(handle, &c)| (handle, Component::Explosion(c)));
        let healths = self.health.healths.iter()
            .map(|(handle, &c)| (handle, Component::Health(c)));
        let missiles = self.missiles.missiles.iter()
            .map(|(handle, &c)| (handle, Component::Missile(c)));
        let ships = self.ships.ships.iter()
            .map(|(handle, &c)| (handle, Component::Ship(c)));

        bodies
            .chain(crafts)
            .chain(explosions)
            .chain(healths)
            .chain(missiles)
            .chain(ships)
    }

    pub fn component_removed(&mut self) -> events::Source<ComponentRemoved> {
        for handle in self.physics.bodies.removed().ready() {
            let handle = ComponentHandle::Body(handle);
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.crafts.crafts.removed().ready() {
            let handle = ComponentHandle::Craft(handle);
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.explosions.explosions.removed().ready() {
            let handle = ComponentHandle::Explosion(handle);
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.health.healths.removed().ready() {
            let handle = ComponentHandle::Health(handle);
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.missiles.missiles.removed().ready() {
            let handle = ComponentHandle::Missile(handle);
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.ships.ships.removed().ready() {
            let handle = ComponentHandle::Ship(handle);
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }

        self.base.component_removed.source()
    }

    pub fn player_created(&mut self) -> events::Source<PlayerCreated> {
        self.players.player_created.source()
    }

    pub fn diagnostics(&self) -> Diagnostics {
        Diagnostics {
            num_bodies:     self.physics.bodies.len()        as u64,
            num_crafts:     self.crafts.crafts.len()         as u64,
            num_explosions: self.explosions.explosions.len() as u64,
            num_healths:    self.health.healths.len()        as u64,
            num_players:    self.players.players.len()       as u64,
            num_missiles:   self.missiles.missiles.len()     as u64,
            num_ships:      self.ships.ships.len()           as u64,
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
