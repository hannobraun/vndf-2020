use toadster::Store;
use vndf_events as events;

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
    pub guidances: Store<Guidance>,
    pub missiles:  Store<Missile>,
    pub targets:   Store<Target>,

    pub missile_launch: events::Buf<MissileLaunch>,

    acc: f32,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            guidances: Store::new(),
            missiles:  Store::new(),
            targets:   Store::new(),

            missile_launch: events::Buf::new(),

            acc: 0.0,
        }
    }

    pub fn on_update(&mut self,
        event:      &Update,
        bodies:     &mut Store<Body>,
        crafts:     &Store<Craft>,
        directions: &mut Store<Direction>,
        fuels:      &Store<Fuel>,
        healths:    &mut Store<Health>,
        positions:  &Store<Position>,
        velocities: &Store<Velocity>,
    ) {
        self.acc += event.dt;

        const TIME: f32 = 0.1;
        if self.acc >= TIME {
            self.acc -= TIME;

            update_targets(
                bodies,
                crafts,
                positions,
                &mut self.targets,
            );
            update_guidances(
                bodies,
                crafts,
                directions,
                &mut self.guidances,
                positions,
                &self.targets,
                velocities,
            );
        }
        explode_missiles(
            bodies,
            crafts,
            fuels,
            &self.guidances,
            healths,
            positions,
            &self.targets,
        );
    }

    pub fn on_missile_launch(&mut self,
        event:      MissileLaunch,
        bodies:     &mut Store<Body>,
        crafts:     &mut Store<Craft>,
        directions: &mut Store<Direction>,
        fuels:      &mut Store<Fuel>,
        healths:    &mut Store<Health>,
        positions:  &mut Store<Position>,
        velocities: &mut Store<Velocity>,
    ) {
        launch_missile(
            bodies,
            crafts,
            directions,
            fuels,
            &mut self.guidances,
            healths,
            &mut self.missiles,
            positions,
            &mut self.targets,
            velocities,
            event.missile,
        );
    }
}
