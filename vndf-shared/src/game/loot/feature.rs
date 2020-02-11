use crate::{
    cgs::Store,
    game::{
        base::Update,
        physics::{
            Body,
            Direction,
            Position,
            Velocity,
        },
    },
};

use super::{
    Loot,
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
        directions: &mut Store<Direction>,
        positions:  &mut Store<Position>,
        velocities: &mut Store<Velocity>,
    ) {
        spawn_loot(
            event.dt,
            bodies,
            directions,
            &mut self.loots,
            positions,
            velocities,
        );
    }
}
