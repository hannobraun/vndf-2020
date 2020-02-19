use std::collections::HashSet;

use toadster::{
    handle::{
        self,
        Untyped,
    },
    store,
};

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
    pub loots: store::Strong<Loot>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            loots: store::Strong::new(),
        }
    }

    pub fn on_update(&mut self,
        event:      &Update,
        bodies:     &mut store::Strong<Body>,
        crafts:     &store::Strong<Craft>,
        directions: &mut store::Strong<Direction>,
        fuels:      &mut store::Strong<Fuel>,
        healths:    &mut store::Strong<Health>,
        positions:  &mut store::Strong<Position>,
        ships:      &mut store::Strong<Ship>,
        velocities: &mut store::Strong<Velocity>,
        index:      &mut HashSet<handle::Strong<Untyped>>,
    ) {
        spawn_random_loot(
            event.dt,
            bodies,
            directions,
            healths,
            &mut self.loots,
            positions,
            velocities,
            index,
        );
        collect_loot(
            bodies,
            crafts,
            fuels,
            healths,
            &mut self.loots,
            positions,
            ships,
        );
    }

    pub fn on_death(&mut self,
        event:      &Death,
        bodies:     &mut store::Strong<Body>,
        crafts:     &store::Strong<Craft>,
        directions: &mut store::Strong<Direction>,
        fuels:      &store::Strong<Fuel>,
        healths:    &mut store::Strong<Health>,
        positions:  &mut store::Strong<Position>,
        ships:      &store::Strong<Ship>,
        velocities: &mut store::Strong<Velocity>,
        index:      &mut HashSet<handle::Strong<Untyped>>,
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
            index,
        );
    }
}
