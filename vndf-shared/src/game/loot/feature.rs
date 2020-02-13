use toadster::StrongStore;

use crate::game::{
    base::Update,
    crafts::{
        Craft,
        Fuel,
    },
    health::{
        Death,
        Health,
    },
    physics::{
        Body,
        Direction,
        Position,
        Velocity,
    },
    ships::Ship,
};

use super::{
    Loot,
    collect_loot,
    spawn_death_loot,
    spawn_random_loot,
};


pub struct Feature {
    pub loots: StrongStore<Loot>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            loots: StrongStore::new(),
        }
    }

    pub fn on_update(&mut self,
        event:      &Update,
        bodies:     &mut StrongStore<Body>,
        crafts:     &StrongStore<Craft>,
        directions: &mut StrongStore<Direction>,
        fuels:      &mut StrongStore<Fuel>,
        positions:  &mut StrongStore<Position>,
        ships:      &mut StrongStore<Ship>,
        velocities: &mut StrongStore<Velocity>,
    ) {
        self.loots.apply_changes();

        spawn_random_loot(
            event.dt,
            bodies,
            directions,
            &mut self.loots,
            positions,
            velocities,
        );
        collect_loot(
            bodies,
            crafts,
            fuels,
            &mut self.loots,
            positions,
            ships,
        );
    }

    pub fn on_death(&mut self,
        event:      &Death,
        bodies:     &mut StrongStore<Body>,
        crafts:     &StrongStore<Craft>,
        directions: &mut StrongStore<Direction>,
        fuels:      &StrongStore<Fuel>,
        healths:    &StrongStore<Health>,
        positions:  &mut StrongStore<Position>,
        ships:      &StrongStore<Ship>,
        velocities: &mut StrongStore<Velocity>,
    ) {
        spawn_death_loot(
            &event.handle,
            bodies,
            crafts,
            directions,
            fuels,
            healths,
            &mut self.loots,
            positions,
            ships,
            velocities,
        );
    }
}
