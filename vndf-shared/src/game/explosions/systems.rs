use std::collections::HashSet;

use rinnsal::EventSink;
use toadster::{
    handle,
    store,
};

use crate::{
    data::ClientHandle,
    game::{
        health::Health,
        physics::{
            Body,
            Position,
            Velocity,
        },
    },
};

use super::{
    Explosion,
    ExplosionEntity,
    ExplosionFaded,
    ExplosionImminent,
};


pub fn explode_entity(
    handle:  &handle::Strong<Health>,
    bodies:  &store::Strong<Body>,
    healths: &store::Strong<Health>,
)
    -> Option<ExplosionEntity>
{
    let health = healths.get(handle)?;
    let body   = bodies.get(&health.body)?;

    let mut is_missile = false;
    if let ClientHandle::Missile(_) = health.parent_ref()? {
        is_missile = true;
    }

    let mut is_ship = false;
    if let ClientHandle::Ship(_) = health.parent_ref()? {
        is_ship = true;
    }

    let mut strength = 0.0;
    if is_missile {
        strength += 3.0;
    }
    if is_ship {
        strength += 6.0;
    }

    Some(ExplosionEntity { exploding: body.clone(), strength })
}

pub fn create_explosion(
    explosion:          ExplosionEntity,
    explosions:         &mut store::Strong<Explosion>,
    positions:          &mut store::Strong<Position>,
    velocities:         &mut store::Strong<Velocity>,
    explosion_imminent: &mut EventSink<ExplosionImminent>,
    index:              &mut HashSet<handle::Strong<Explosion>>,
) {
    let handle = explosion.create(
        explosions,
        positions,
        velocities,
        index,
    );
    if let Some(handle) = handle {
        explosion_imminent.push(ExplosionImminent { handle });
    }
}

pub fn damage_nearby(
    handle:     &handle::Strong<Explosion>,
    bodies:     &store::Strong<Body>,
    explosions: &store::Strong<Explosion>,
    healths:    &mut store::Strong<Health>,
    positions:  &store::Strong<Position>,
)
    -> Option<()>
{
    let explosion = explosions.get(handle)?;
    let position  = positions.get(&explosion.pos)?;

    let nearby = healths.values_mut()
        .into_iter()
        .filter_map(|health| {
            let body = bodies.get(&health.body)?;
            let pos  = positions.get(&body.pos)?;
            Some((pos, health))
        });

    explosion.damage_nearby(&position, nearby);
    Some(())
}

pub fn update_explosions(
    explosions:      &mut store::Strong<Explosion>,
    dt:              f32,
    explosion_faded: &mut EventSink<ExplosionFaded>,
) {
    for (handle, explosion) in explosions.iter_mut().strong() {
        if explosion.update(dt) {
            explosion_faded.push(ExplosionFaded { handle });
        }
    }
}
