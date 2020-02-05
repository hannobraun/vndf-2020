use crate::{
    cgs::Store,
    game::{
        crafts::Craft,
        health::Health,
        physics::{
            Body,
            Position,
            Velocity,
        },
    },
    events,
};

use super::{
    Missile,
    MissileLaunch,
    Target,
    explode_missiles,
    launch_missile,
    update_guidances,
    update_targets,
};


pub struct Feature {
    pub missiles: Store<Missile>,
    pub targets:  Store<Target>,

    pub missile_launch: events::Buf<MissileLaunch>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            missiles: Store::new(),
            targets:  Store::new(),

            missile_launch: events::Buf::new(),
        }
    }

    pub fn on_update(&mut self,
        bodies:     &mut Store<Body>,
        crafts:     &Store<Craft>,
        healths:    &mut Store<Health>,
        positions:  &Store<Position>,
        velocities: &Store<Velocity>,
    ) {
        update_targets(
            bodies,
            crafts,
            positions,
            &mut self.targets,
        );
        update_guidances(
            bodies,
            crafts,
            &mut self.missiles,
            positions,
            &self.targets,
            velocities,
        );
        explode_missiles(
            bodies,
            crafts,
            healths,
            &self.missiles,
            positions,
            &self.targets,
        );
    }

    pub fn on_missile_launch(&mut self,
        event:      MissileLaunch,
        bodies:     &mut Store<Body>,
        crafts:     &mut Store<Craft>,
        healths:    &mut Store<Health>,
        positions:  &mut Store<Position>,
        velocities: &mut Store<Velocity>,
    ) {
        launch_missile(
            bodies,
            crafts,
            healths,
            &mut self.missiles,
            positions,
            &mut self.targets,
            velocities,
            event.missile,
        );
    }
}
