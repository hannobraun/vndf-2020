use bach::EventSink;
use toadster::{
    StrongHandle,
    StrongStore,
};

use crate::game::{
    base::ComponentHandle,
    health::Health,
    physics::{
        Body,
        Direction,
        Position,
        Velocity,
    },
};

use super::{
    Explosion,
    ExplosionEntity,
    ExplosionFaded,
    ExplosionImminent,
};


pub fn explode_entity(
    handle:  &StrongHandle<Health>,
    bodies:  &StrongStore<Body>,
    healths: &StrongStore<Health>,
)
    -> Option<ExplosionEntity>
{
    let health = healths.get(handle)?;
    let body   = bodies.get(&health.body)?;

    let mut is_missile = false;
    if let ComponentHandle::Missile(_) = health.parent.as_ref()? {
        is_missile = true;
    }

    let mut is_ship = false;
    if let ComponentHandle::Ship(_) = &health.parent.as_ref()? {
        is_ship = true;
    }

    let mut strength = 0.0;
    if is_missile {
        strength += 3.0;
    }
    if is_ship {
        strength += 6.0;
    }

    Some(ExplosionEntity { exploding: *body, strength })
}

pub fn create_explosion(
    bodies:             &mut StrongStore<Body>,
    directions:         &mut StrongStore<Direction>,
    explosions:         &mut StrongStore<Explosion>,
    positions:          &mut StrongStore<Position>,
    velocities:         &mut StrongStore<Velocity>,
    explosion_imminent: &mut EventSink<ExplosionImminent>,
    explosion:          ExplosionEntity,
) {
    let handle = explosion.create(
        bodies,
        directions,
        explosions,
        positions,
        velocities,
    );
    if let Some(handle) = handle {
        explosion_imminent.push(ExplosionImminent { handle });
    }
}

pub fn damage_nearby(
    handle:     &StrongHandle<Explosion>,
    bodies:     &StrongStore<Body>,
    explosions: &StrongStore<Explosion>,
    healths:    &mut StrongStore<Health>,
    positions:  &StrongStore<Position>,
)
    -> Option<()>
{
    let explosion = explosions.get(handle)?;
    let body      = bodies.get(&explosion.body)?;
    let position  = positions.get(&body.pos)?;

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
    explosions:      &mut StrongStore<Explosion>,
    dt:              f32,
    explosion_faded: &mut EventSink<ExplosionFaded>,
) {
    for (handle, explosion) in explosions {
        if explosion.update(dt) {
            explosion_faded.push(ExplosionFaded { handle });
        }
    }
}

pub fn remove_explosion(
    handle:     StrongHandle<Explosion>,
    bodies:     &mut StrongStore<Body>,
    directions: &mut StrongStore<Direction>,
    explosions: &mut StrongStore<Explosion>,
    positions:  &mut StrongStore<Position>,
    velocities: &mut StrongStore<Velocity>,
)
    -> Option<()>
{
    let explosion = explosions.remove(handle)?;
    Body::remove(
        explosion.body,
        bodies,
        directions,
        positions,
        velocities,
    );
    Some(())
}
