use crate::{
    cgs::Store,
    game::{
        crafts::Craft,
        health::Health,
        physics::{
            Body,
            Position,
        },
    },
    events,
};

use super::{
    Missile,
    MissileLaunch,
    explode_missiles,
    launch_missile,
    update_guidances,
    update_targets,
};


pub struct Feature {
    pub missiles:       Store<Missile>,
    pub missile_launch: events::Buf<MissileLaunch>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            missiles:       Store::new(),
            missile_launch: events::Buf::new(),
        }
    }

    pub fn on_update(&mut self,
        bodies:    &mut Store<Body>,
        crafts:    &Store<Craft>,
        healths:   &mut Store<Health>,
        positions: &Store<Position>,
    ) {
        update_targets(
            bodies,
            crafts,
            &mut self.missiles,
            positions,
        );
        update_guidances(
            bodies,
            crafts,
            &mut self.missiles,
            positions,
        );
        explode_missiles(
            bodies,
            crafts,
            healths,
            &self.missiles,
            positions,
        );
    }

    pub fn on_missile_launch(&mut self,
        event:     MissileLaunch,
        bodies:    &mut Store<Body>,
        crafts:    &mut Store<Craft>,
        healths:   &mut Store<Health>,
        positions: &mut Store<Position>,
    ) {
        launch_missile(
            bodies,
            crafts,
            healths,
            &mut self.missiles,
            positions,
            event.missile,
        );
    }
}
