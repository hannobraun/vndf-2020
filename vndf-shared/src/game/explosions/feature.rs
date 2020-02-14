use bach::EventBuf;
use toadster::StrongStore;

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
    remove_explosion,
    update_explosions,
};


pub struct Feature {
    pub explosions: StrongStore<Explosion>,

    pub explosion_faded:    EventBuf<ExplosionFaded>,
    pub explosion_imminent: EventBuf<ExplosionImminent>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            explosions: StrongStore::new(),

            explosion_faded:    EventBuf::new(),
            explosion_imminent: EventBuf::new(),
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
        bodies:     &mut StrongStore<Body>,
        directions: &mut StrongStore<Direction>,
        healths:    &StrongStore<Health>,
        positions:  &mut StrongStore<Position>,
        velocities: &mut StrongStore<Velocity>,
    ) {
        let explosion = explode_entity(
            &event.handle,
            bodies,
            healths,
        );
        if let Some(explosion) = explosion {
            create_explosion(
                bodies,
                directions,
                &mut self.explosions,
                positions,
                velocities,
                &mut self.explosion_imminent.sink(),
                explosion,
            );
        }
    }

    pub fn on_explosion_imminent(&self,
        event:     &ExplosionImminent,
        bodies:    &StrongStore<Body>,
        healths:   &mut StrongStore<Health>,
        positions: &StrongStore<Position>,
    ) {
        damage_nearby(
            &event.handle,
            &bodies,
            &self.explosions,
            healths,
            positions,
        );
    }

    pub fn on_explosion_faded(&mut self,
        event:      &ExplosionFaded,
        bodies:     &mut StrongStore<Body>,
        directions: &mut StrongStore<Direction>,
        positions:  &mut StrongStore<Position>,
        velocities: &mut StrongStore<Velocity>,
    ) {
        remove_explosion(
            event.handle,
            bodies,
            directions,
            &mut self.explosions,
            positions,
            velocities,
        );
    }
}
