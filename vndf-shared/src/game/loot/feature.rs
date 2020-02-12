use crate::{
    cgs::Store,
    game::{
        base::Update,
        crafts::{
            Craft,
            Fuel,
        },
        physics::{
            Body,
            Direction,
            Position,
            Velocity,
        },
        ships::Ship,
    },
};

use super::{
    Loot,
    collect_loot,
    spawn_loot,
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

        spawn_loot(
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
}
