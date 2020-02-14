use bach::EventSink;
use toadster::{
    handle,
    store,
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
    handle:  &handle::Strong<Health>,
    bodies:  &store::Strong<Body>,
    healths: &store::Strong<Health>,
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

    Some(ExplosionEntity { exploding: body.clone(), strength })
}

pub fn create_explosion(
    bodies:             &mut store::Strong<Body>,
    directions:         &mut store::Strong<Direction>,
    explosions:         &mut store::Strong<Explosion>,
    positions:          &mut store::Strong<Position>,
    velocities:         &mut store::Strong<Velocity>,
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
    handle:     &handle::Strong<Explosion>,
    bodies:     &store::Strong<Body>,
    explosions: &store::Strong<Explosion>,
    healths:    &mut store::Strong<Health>,
    positions:  &store::Strong<Position>,
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
    explosions:      &mut store::Strong<Explosion>,
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
    handle:     handle::Strong<Explosion>,
    bodies:     &mut store::Strong<Body>,
    directions: &mut store::Strong<Direction>,
    explosions: &mut store::Strong<Explosion>,
    positions:  &mut store::Strong<Position>,
    velocities: &mut store::Strong<Velocity>,
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
