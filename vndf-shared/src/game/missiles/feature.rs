use toadster::StrongStore;
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
    pub guidances: StrongStore<Guidance>,
    pub missiles:  StrongStore<Missile>,
    pub targets:   StrongStore<Target>,

    pub missile_launch: events::Buf<MissileLaunch>,

    acc: f32,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            guidances: StrongStore::new(),
            missiles:  StrongStore::new(),
            targets:   StrongStore::new(),

            missile_launch: events::Buf::new(),

            acc: 0.0,
        }
    }

    pub fn on_update(&mut self,
        event:      &Update,
        bodies:     &mut StrongStore<Body>,
        crafts:     &StrongStore<Craft>,
        directions: &mut StrongStore<Direction>,
        fuels:      &StrongStore<Fuel>,
        healths:    &mut StrongStore<Health>,
        positions:  &StrongStore<Position>,
        velocities: &StrongStore<Velocity>,
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
        bodies:     &mut StrongStore<Body>,
        crafts:     &mut StrongStore<Craft>,
        directions: &mut StrongStore<Direction>,
        fuels:      &mut StrongStore<Fuel>,
        healths:    &mut StrongStore<Health>,
        positions:  &mut StrongStore<Position>,
        velocities: &mut StrongStore<Velocity>,
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
