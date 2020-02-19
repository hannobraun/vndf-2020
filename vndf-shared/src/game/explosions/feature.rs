use std::collections::HashSet;

use rinnsal::EventBuf;
use toadster::{
    handle,
    store,
};

use crate::game::{
    base::Update,
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
};

use super::{
    Explosion,
    ExplosionFaded,
    ExplosionImminent,
    create_explosion,
    damage_nearby,
    explode_entity,
    update_explosions,
};


pub struct Feature {
    pub explosions: store::Strong<Explosion>,

    pub explosion_faded:    EventBuf<ExplosionFaded>,
    pub explosion_imminent: EventBuf<ExplosionImminent>,

    pub index: HashSet<handle::Strong<Explosion>>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            explosions: store::Strong::new(),

            explosion_faded:    EventBuf::new(),
            explosion_imminent: EventBuf::new(),

            index: HashSet::new(),
        }
    }

    pub fn on_update(&mut self, event: &Update) {
        update_explosions(
            &mut self.explosions,
            event.dt,
            &mut self.explosion_faded.sink(),
        );
    }

    pub fn on_death(&mut self,
        event:      &Death,
        bodies:     &mut store::Strong<Body>,
        directions: &mut store::Strong<Direction>,
        healths:    &store::Strong<Health>,
        positions:  &mut store::Strong<Position>,
        velocities: &mut store::Strong<Velocity>,
    ) {
        let explosion = explode_entity(
            &event.handle,
            bodies,
            healths,
        );
        if let Some(explosion) = explosion {
            create_explosion(
                explosion,
                bodies,
                directions,
                &mut self.explosions,
                positions,
                velocities,
                &mut self.explosion_imminent.sink(),
                &mut self.index,
            );
        }
    }

    pub fn on_explosion_imminent(&self,
        event:     &ExplosionImminent,
        bodies:    &store::Strong<Body>,
        healths:   &mut store::Strong<Health>,
        positions: &store::Strong<Position>,
    ) {
        damage_nearby(
            &event.handle,
            &bodies,
            &self.explosions,
            healths,
            positions,
        );
    }

    pub fn on_explosion_faded(&mut self, event: &ExplosionFaded) {
        self.index.remove(&event.handle);
    }
}
