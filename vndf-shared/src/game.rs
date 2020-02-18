pub mod base;
pub mod crafts;
pub mod explosions;
pub mod health;
pub mod loot;
pub mod missiles;
pub mod physics;
pub mod players;
pub mod ships;


use rinnsal::{
    EventSink,
    EventSource,
};
use serde::{
    Deserialize,
    Serialize,
};

use self::{
    base::{
        Component,
        ComponentHandle,
        ComponentRemoved,
        Update,
    },
    players::{
        InputHandled,
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
    loot:       loot::Feature,
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
            loot:       loot::Feature::new(),
            missiles:   missiles::Feature::new(),
            physics:    physics::Feature::new(),
            players:    players::Feature::new(),
            ships:      ships::Feature::new(),
        }
    }

    pub fn player_connected(&mut self) -> EventSink<PlayerConnected> {
        self.players.player_connected.sink()
    }

    pub fn player_disconnected(&mut self) -> EventSink<PlayerDisconnected> {
        self.players.player_disconnected.sink()
    }

    pub fn player_input(&mut self) -> EventSink<PlayerInput> {
        self.players.player_input.sink()
    }

    pub fn update(&mut self) -> EventSink<Update> {
        self.base.update.sink()
    }

    pub fn dispatch(&mut self) {
        for event in self.base.update.source().ready() {
            self.crafts.crafts.apply_changes();
            self.crafts.fuels.apply_changes();
            self.explosions.explosions.apply_changes();
            self.health.healths.apply_changes();
            self.loot.loots.apply_changes();
            self.missiles.guidances.apply_changes();
            self.missiles.missiles.apply_changes();
            self.missiles.targets.apply_changes();
            self.physics.bodies.apply_changes();
            self.physics.directions.apply_changes();
            self.physics.positions.apply_changes();
            self.physics.velocities.apply_changes();
            self.players.players.apply_changes();
            self.ships.ships.apply_changes();

            self.crafts.on_update(
                &event,
                &mut self.physics.bodies,
                &self.physics.directions,
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
                &event,
                &mut self.physics.bodies,
                &self.crafts.crafts,
                &mut self.physics.directions,
                &self.crafts.fuels,
                &mut self.health.healths,
                &self.physics.positions,
                &self.physics.velocities,
            );
            self.ships.on_update(
                &mut self.physics.bodies,
                &self.crafts.crafts,
            );
            self.loot.on_update(
                &event,
                &mut self.physics.bodies,
                &self.crafts.crafts,
                &mut self.physics.directions,
                &mut self.crafts.fuels,
                &mut self.physics.positions,
                &mut self.ships.ships,
                &mut self.physics.velocities,
            );
        }
        while let Some(event) = self.players.player_connected.source().next() {
            self.players.on_player_connected(
                &event,
                &mut self.physics.bodies,
                &mut self.crafts.crafts,
                &mut self.physics.directions,
                &mut self.crafts.fuels,
                &mut self.health.healths,
                &mut self.physics.positions,
                &mut self.ships.ships,
                &mut self.physics.velocities,
                &mut self.health.index,
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
                &mut self.physics.directions,
                &mut self.crafts.fuels,
                &mut self.health.healths,
                &mut self.physics.positions,
                &mut self.physics.velocities,
                &mut self.health.index,
            );
        }
        while let Some(event) = self.health.death.source().next() {
            self.explosions.on_death(
                &event,
                &mut self.physics.bodies,
                &mut self.physics.directions,
                &self.health.healths,
                &mut self.physics.positions,
                &mut self.physics.velocities,
            );
            self.loot.on_death(
                &event,
                &mut self.physics.bodies,
                &self.crafts.crafts,
                &mut self.physics.directions,
                &self.crafts.fuels,
                &self.health.healths,
                &mut self.physics.positions,
                &self.ships.ships,
                &mut self.physics.velocities,
            );
            self.health.on_death(
                &event,
                &mut self.physics.bodies,
                &mut self.crafts.crafts,
                &mut self.physics.directions,
                &mut self.crafts.fuels,
                &mut self.missiles.guidances,
                &mut self.missiles.missiles,
                &mut self.physics.positions,
                &mut self.ships.ships,
                &mut self.missiles.targets,
                &mut self.physics.velocities,
            );
        }
        while let Some(event) =
            self.explosions.explosion_imminent.source().next()
        {
            self.explosions.on_explosion_imminent(
                &event,
                &self.physics.bodies,
                &mut self.health.healths,
                &self.physics.positions,
            )
        }
        while let Some(event) =
            self.explosions.explosion_faded.source().next()
        {
            self.explosions.on_explosion_faded(
                &event,
                &mut self.physics.bodies,
                &mut self.physics.directions,
                &mut self.physics.positions,
                &mut self.physics.velocities,
            );
        }
    }

    pub fn updates(&mut self) -> impl Iterator<Item=Component> + '_ {
        let bodies = self.physics.bodies
            .iter()
            .map(|(handle, c)|
                Component::Body(handle.into_weak().into(), c.clone())
            );
        let crafts = self.crafts.crafts
            .iter()
            .map(|(handle, c)|
                Component::Craft(handle.into_weak().into(), c.clone())
            );
        let directions = self.physics.directions
            .iter()
            .map(|(handle, c)|
                Component::Direction(handle.into_weak().into(), c.clone())
            );
        let explosions = self.explosions.explosions
            .iter()
            .map(|(handle, c)|
                Component::Explosion(handle.into_weak().into(), c.clone())
            );
        let fuels = self.crafts.fuels
            .iter()
            .map(|(handle, c)|
                Component::Fuel(handle.into_weak().into(), c.clone())
            );
        let healths = self.health.healths
            .iter()
            .map(|(handle, c)|
                Component::Health(handle.into_weak().into(), c.clone())
            );
        let loots = self.loot.loots
            .iter()
            .map(|(handle, c)|
                Component::Loot(handle.into_weak().into(), c.clone())
            );
        let missiles = self.missiles.missiles
            .iter()
            .map(|(handle, c)|
                Component::Missile(handle.into_weak().into(), c.clone())
            );
        let positions = self.physics.positions
            .iter()
            .map(|(handle, c)|
                Component::Position(handle.into_weak().into(), c.clone())
            );
        let ships = self.ships.ships
            .iter()
            .map(|(handle, c)|
                Component::Ship(handle.into_weak().into(), c.clone())
            );
        let targets = self.missiles.targets
            .iter()
            .map(|(handle, c)|
                Component::Target(handle.into_weak().into(), c.clone())
            );
        let velocities = self.physics.velocities
            .iter()
            .map(|(handle, c)|
                Component::Velocity(handle.into_weak().into(), c.clone())
            );

        bodies
            .chain(crafts)
            .chain(directions)
            .chain(explosions)
            .chain(fuels)
            .chain(healths)
            .chain(loots)
            .chain(missiles)
            .chain(positions)
            .chain(ships)
            .chain(targets)
            .chain(velocities)
    }

    pub fn removals(&mut self) -> EventSource<ComponentRemoved> {
        for handle in self.physics.bodies.removed().ready() {
            let handle = ComponentHandle::Body(handle.into());
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.crafts.crafts.removed().ready() {
            let handle = ComponentHandle::Craft(handle.into());
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.physics.directions.removed().ready() {
            let handle = ComponentHandle::Direction(handle.into());
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.explosions.explosions.removed().ready() {
            let handle = ComponentHandle::Explosion(handle.into());
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.crafts.fuels.removed().ready() {
            let handle = ComponentHandle::Fuel(handle.into());
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.health.healths.removed().ready() {
            let handle = ComponentHandle::Health(handle.into());
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.loot.loots.removed().ready() {
            let handle = ComponentHandle::Loot(handle.into());
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.missiles.missiles.removed().ready() {
            let handle = ComponentHandle::Missile(handle.into());
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.physics.positions.removed().ready() {
            let handle = ComponentHandle::Position(handle.into());
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.ships.ships.removed().ready() {
            let handle = ComponentHandle::Ship(handle.into());
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.missiles.targets.removed().ready() {
            let handle = ComponentHandle::Target(handle.into());
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.physics.velocities.removed().ready() {
            let handle = ComponentHandle::Velocity(handle.into());
            let event  = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }

        self.base.component_removed.source()
    }

    pub fn player_created(&mut self) -> EventSource<PlayerCreated> {
        self.players.player_created.source()
    }

    pub fn input_handled(&mut self) -> EventSource<InputHandled> {
        self.players.input_handled.source()
    }

    pub fn diagnostics(&self) -> Diagnostics {
        Diagnostics {
            num_bodies:     self.physics.bodies.len()        as u64,
            num_crafts:     self.crafts.crafts.len()         as u64,
            num_directions: self.physics.directions.len()    as u64,
            num_explosions: self.explosions.explosions.len() as u64,
            num_fuels:      self.crafts.fuels.len()          as u64,
            num_guidances:  self.missiles.guidances.len()    as u64,
            num_healths:    self.health.healths.len()        as u64,
            num_loots:      self.loot.loots.len()            as u64,
            num_players:    self.players.players.len()       as u64,
            num_missiles:   self.missiles.missiles.len()     as u64,
            num_positions:  self.physics.positions.len()     as u64,
            num_ships:      self.ships.ships.len()           as u64,
            num_targets:    self.missiles.targets.len()      as u64,
            num_velocities: self.physics.velocities.len()    as u64,
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Diagnostics {
    pub num_bodies:     u64,
    pub num_crafts:     u64,
    pub num_directions: u64,
    pub num_explosions: u64,
    pub num_fuels:      u64,
    pub num_guidances:  u64,
    pub num_healths:    u64,
    pub num_loots:      u64,
    pub num_players:    u64,
    pub num_missiles:   u64,
    pub num_positions:  u64,
    pub num_ships:      u64,
    pub num_targets:    u64,
    pub num_velocities: u64,
}
