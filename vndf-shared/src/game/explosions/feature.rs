use crate::{
    cgs::Store,
    events,
    game::{
        base::Update,
        health::{
            Death,
            Health,
        },
        physics::{
            Body,
            Position,
            Velocity,
        },
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
    pub explosions: Store<Explosion>,

    pub explosion_faded:    events::Buf<ExplosionFaded>,
    pub explosion_imminent: events::Buf<ExplosionImminent>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            explosions: Store::new(),

            explosion_faded:    events::Buf::new(),
            explosion_imminent: events::Buf::new(),
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
        bodies:     &mut Store<Body>,
        healths:    &Store<Health>,
        positions:  &mut Store<Position>,
        velocities: &mut Store<Velocity>,
    ) {
        let explosion = explode_entity(
            bodies,
            healths,
            event.handle,
        );
        if let Some(explosion) = explosion {
            create_explosion(
                bodies,
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
        bodies:    &Store<Body>,
        healths:   &mut Store<Health>,
        positions: &Store<Position>,
    ) {
        damage_nearby(
            event.handle,
            &bodies,
            &self.explosions,
            healths,
            positions,
        );
    }

    pub fn on_explosion_faded(&mut self,
        event:      &ExplosionFaded,
        bodies:     &mut Store<Body>,
        positions:  &mut Store<Position>,
        velocities: &mut Store<Velocity>,
    ) {
        remove_explosion(
            event.handle,
            bodies,
            &mut self.explosions,
            positions,
            velocities,
        );
    }
}
