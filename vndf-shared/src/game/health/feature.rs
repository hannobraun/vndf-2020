use std::collections::HashSet;

use rinnsal::EventBuf;
use toadster::{
    handle::{
        self,
        Untyped,
    },
    store,
};

use crate::game::{
    crafts::{
        Craft,
        Fuel,
    },
    loot::Loot,
    missiles::{
        Guidance,
        Missile,
        Target,
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
    Death,
    Health,
    check_health,
    remove_entity,
};


pub struct Feature {
    pub healths: store::Strong<Health>,
    pub death:   EventBuf<Death>,
    pub index:   HashSet<handle::Strong<Untyped>>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            healths: store::Strong::new(),
            death:   EventBuf::new(),
            index:   HashSet::new(),
        }
    }

    pub fn on_update(&mut self) {
        check_health(
            &self.healths,
            &mut self.death.sink(),
            &mut self.index,
        );
    }

    pub fn on_death(&mut self,
        event:      &Death,
        bodies:     &mut store::Strong<Body>,
        crafts:     &mut store::Strong<Craft>,
        directions: &mut store::Strong<Direction>,
        fuels:      &mut store::Strong<Fuel>,
        guidances:  &mut store::Strong<Guidance>,
        loots:      &mut store::Strong<Loot>,
        missiles:   &mut store::Strong<Missile>,
        positions:  &mut store::Strong<Position>,
        ships:      &mut store::Strong<Ship>,
        targets:    &mut store::Strong<Target>,
        velocities: &mut store::Strong<Velocity>,
    )
        -> Option<()>
    {
        remove_entity(
            event.handle.clone(),
            bodies,
            crafts,
            directions,
            fuels,
            guidances,
            &mut self.healths,
            loots,
            missiles,
            positions,
            ships,
            targets,
            velocities,
        );

        Some(())
    }
}
