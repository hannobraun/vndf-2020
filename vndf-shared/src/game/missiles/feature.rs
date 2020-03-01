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
    base::Update,
    crafts::{
        Craft,
        Fuel,
    },
    health::Health,
    physics::{
        Body,
        Direction,
        Position,
        Velocity,
    },
};

use super::{
    Guidance,
    Missile,
    MissileLaunch,
    Target,
    explode_missiles,
    launch_missile,
    update_guidances,
    update_targets,
};


pub struct Feature {
    pub missile_launch: EventBuf<MissileLaunch>,

    acc: f32,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            missile_launch: EventBuf::new(),

            acc: 0.0,
        }
    }

    pub fn on_update(&mut self,
        event:      &Update,
        bodies:     &mut store::Strong<Body>,
        crafts:     &store::Strong<Craft>,
        directions: &mut store::Strong<Direction>,
        fuels:      &store::Strong<Fuel>,
        guidances:  &mut store::Strong<Guidance>,
        healths:    &mut store::Strong<Health>,
        positions:  &store::Strong<Position>,
        targets:    &mut store::Strong<Target>,
        velocities: &store::Strong<Velocity>,
    ) {
        self.acc += event.dt;

        const TIME: f32 = 0.1;
        if self.acc >= TIME {
            self.acc -= TIME;

            update_targets(
                bodies,
                crafts,
                positions,
                targets,
            );
            update_guidances(
                bodies,
                crafts,
                directions,
                guidances,
                positions,
                targets,
                velocities,
            );
        }
        explode_missiles(
            bodies,
            crafts,
            fuels,
            guidances,
            healths,
            positions,
            targets,
        );
    }

    pub fn on_missile_launch(&mut self,
        event:      MissileLaunch,
        bodies:     &mut store::Strong<Body>,
        crafts:     &mut store::Strong<Craft>,
        directions: &mut store::Strong<Direction>,
        fuels:      &mut store::Strong<Fuel>,
        guidances:  &mut store::Strong<Guidance>,
        healths:    &mut store::Strong<Health>,
        missiles:   &mut store::Strong<Missile>,
        positions:  &mut store::Strong<Position>,
        targets:    &mut store::Strong<Target>,
        velocities: &mut store::Strong<Velocity>,
        entities:   &mut HashSet<handle::Strong<Untyped>>,
    ) {
        launch_missile(
            event.missile,
            bodies,
            crafts,
            directions,
            fuels,
            guidances,
            healths,
            missiles,
            positions,
            targets,
            velocities,
            entities,
        );
    }
}
