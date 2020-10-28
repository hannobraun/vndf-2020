pub mod features;
pub mod math;

pub use self::math::{Angle, Length, Pnt2, Scalar, Size, Vec2};

use rinnsal::{EventSink, EventSource};
use toadster::handle;

use crate::data;

use self::features::{
    base::{self, ComponentRemoved, Update},
    crafts, explosions, health, physics,
    planets::{self, Planet, Planets},
    players::{
        self, InputHandled, PlayerConnected, PlayerCreated, PlayerDisconnected, PlayerId,
        PlayerInput,
    },
    ships,
};

pub const TARGET_FPS: u32 = 60;
pub const FRAME_TIME: Scalar = 1.0 / TARGET_FPS as Scalar;

pub struct State {
    data: data::server::Components,

    base: base::Feature,
    crafts: crafts::Feature,
    explosions: explosions::Feature,
    health: health::Feature,
    physics: physics::Feature,
    players: players::Feature,
    ships: ships::Feature,

    // Need to keep this handle, otherwise planet will get garbage-collected.
    _planet: handle::Strong<Planet>,
}

impl State {
    pub fn new() -> Self {
        let mut data = data::server::Components::new();

        let planet = data.planets.insert(Planet {
            pos: Pnt2::new(0.0, 0.0),
            radius: Length::new(60_268_000.0), // size of Saturn (in m)
            mass: 5.6834e26,                   // mass of Saturn (in kg)
        });

        Self {
            data,

            base: base::Feature::new(),
            crafts: crafts::Feature::new(),
            explosions: explosions::Feature::new(),
            health: health::Feature::new(),
            physics: physics::Feature::new(),
            players: players::Feature::new(),
            ships: ships::Feature::new(),

            _planet: planet,
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
        self.data.apply_changes();
        for event in self.base.update.source().ready() {
            self.crafts.on_update(
                &event,
                &mut self.data.bodies,
                &mut self.data.crafts,
                &mut self.data.fuels,
            );
            self.explosions.on_update(&event, &mut self.data.explosions);

            let mut planets = planets::Systems {
                bodies: &mut self.data.bodies,
                healths: &mut self.data.healths,
                planets: Planets(&self.data.planets),
                positions: &self.data.positions,
            };
            planets.on_update();

            self.physics.on_update(
                &event,
                &mut self.data.bodies,
                &self.data.planets,
                &mut self.data.positions,
                &mut self.data.velocities,
            );
            self.health.on_update(&self.data.healths);
            self.ships.on_update(
                &mut self.data.bodies,
                &self.data.crafts,
                &mut self.data.ships,
            );
        }
        self.data.apply_changes();
        while let Some(event) = self.players.player_connected.source().next() {
            // We only have one planet right now.
            let planet = self.data.planets.iter().next().unwrap().1;

            self.players.on_player_connected(
                &event,
                planet,
                &mut self.data.bodies,
                &mut self.data.crafts,
                &mut self.data.fuels,
                &mut self.data.healths,
                &mut self.data.players,
                &mut self.data.positions,
                &mut self.data.ships,
                &mut self.data.velocities,
                &mut self.health.index,
            );
        }
        self.data.apply_changes();
        while let Some(event) = self.players.player_disconnected.source().next() {
            self.players.on_player_disconnected(&event);
        }
        self.data.apply_changes();
        while let Some(event) = self.players.player_input.source().next() {
            self.players.on_player_input(
                &event,
                &mut self.data.crafts,
                &self.data.players,
                &mut self.data.ships,
            );
        }
        self.data.apply_changes();
        while let Some(event) = self.health.death.source().next() {
            self.explosions.on_death(
                &event,
                &mut self.data.bodies,
                &mut self.data.explosions,
                &self.data.healths,
                &mut self.data.positions,
                &mut self.data.velocities,
            );
        }
        self.data.apply_changes();
        while let Some(event) = self.explosions.explosion_imminent.source().next() {
            self.explosions.on_explosion_imminent(
                &event,
                &self.data.bodies,
                &self.data.explosions,
                &mut self.data.healths,
                &self.data.positions,
            )
        }
        self.data.apply_changes();
        while let Some(event) = self.explosions.explosion_faded.source().next() {
            self.explosions.on_explosion_faded(&event);
        }
    }

    pub fn updates(&mut self) -> impl Iterator<Item = data::client::Component> + '_ {
        let bodies = self
            .data
            .bodies
            .iter()
            .map(|(handle, c)| data::client::Component::Body(handle.into(), c.to_weak()));
        let crafts = self
            .data
            .crafts
            .iter()
            .map(|(handle, c)| data::client::Component::Craft(handle.into(), c.to_weak()));
        let explosions = self
            .data
            .explosions
            .iter()
            .map(|(handle, c)| data::client::Component::Explosion(handle.into(), c.to_weak()));
        let fuels = self
            .data
            .fuels
            .iter()
            .map(|(handle, c)| data::client::Component::Fuel(handle.into(), c.to_weak()));
        let healths = self
            .data
            .healths
            .iter()
            .map(|(handle, c)| data::client::Component::Health(handle.into(), c.to_weak()));
        let planets = self
            .data
            .planets
            .iter()
            .map(|(handle, c)| data::client::Component::Planet(handle.into(), c.to_weak()));
        let positions = self
            .data
            .positions
            .iter()
            .map(|(handle, c)| data::client::Component::Position(handle.into(), c.to_weak()));
        let ships = self
            .data
            .ships
            .iter()
            .map(|(handle, c)| data::client::Component::Ship(handle.into(), c.to_weak()));
        let velocities = self
            .data
            .velocities
            .iter()
            .map(|(handle, c)| data::client::Component::Velocity(handle.into(), c.to_weak()));

        bodies
            .chain(crafts)
            .chain(explosions)
            .chain(fuels)
            .chain(healths)
            .chain(planets)
            .chain(positions)
            .chain(ships)
            .chain(velocities)
    }

    pub fn removals(&mut self) -> EventSource<ComponentRemoved> {
        for handle in self.data.bodies.removed().ready() {
            let handle = data::client::Handle::Body(handle.into());
            let event = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.data.crafts.removed().ready() {
            let handle = data::client::Handle::Craft(handle.into());
            let event = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.data.explosions.removed().ready() {
            let handle = data::client::Handle::Explosion(handle.into());
            let event = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.data.fuels.removed().ready() {
            let handle = data::client::Handle::Fuel(handle.into());
            let event = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.data.healths.removed().ready() {
            let handle = data::client::Handle::Health(handle.into());
            let event = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.data.positions.removed().ready() {
            let handle = data::client::Handle::Position(handle.into());
            let event = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.data.ships.removed().ready() {
            let handle = data::client::Handle::Ship(handle.into());
            let event = ComponentRemoved { handle };
            self.base.component_removed.sink().push(event);
        }
        for handle in self.data.velocities.removed().ready() {
            let handle = data::client::Handle::Velocity(handle.into());
            let event = ComponentRemoved { handle };
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

    pub fn diagnostics(&self) -> data::server::Diagnostics {
        (&self.data).into()
    }
}
