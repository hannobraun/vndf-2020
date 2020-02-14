use rinnsal::EventBuf;
use toadster::store;

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
    pub guidances: store::Strong<Guidance>,
    pub missiles:  store::Strong<Missile>,
    pub targets:   store::Strong<Target>,

    pub missile_launch: EventBuf<MissileLaunch>,

    acc: f32,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            guidances: store::Strong::new(),
            missiles:  store::Strong::new(),
            targets:   store::Strong::new(),

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
        healths:    &mut store::Strong<Health>,
        positions:  &store::Strong<Position>,
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
        bodies:     &mut store::Strong<Body>,
        crafts:     &mut store::Strong<Craft>,
        directions: &mut store::Strong<Direction>,
        fuels:      &mut store::Strong<Fuel>,
        healths:    &mut store::Strong<Health>,
        positions:  &mut store::Strong<Position>,
        velocities: &mut store::Strong<Velocity>,
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
