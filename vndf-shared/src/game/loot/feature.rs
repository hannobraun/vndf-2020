use toadster::Store;

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
    pub loots: Store<Loot>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            loots: Store::new(),
        }
    }

    pub fn on_update(&mut self,
        event:      &Update,
        bodies:     &mut Store<Body>,
        crafts:     &Store<Craft>,
        directions: &mut Store<Direction>,
        fuels:      &mut Store<Fuel>,
        positions:  &mut Store<Position>,
        ships:      &mut Store<Ship>,
        velocities: &mut Store<Velocity>,
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
        bodies:     &mut Store<Body>,
        crafts:     &Store<Craft>,
        directions: &mut Store<Direction>,
        fuels:      &Store<Fuel>,
        healths:    &Store<Health>,
        positions:  &mut Store<Position>,
        ships:      &Store<Ship>,
        velocities: &mut Store<Velocity>,
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
