use crate::{
    cgs::{
        Handle,
        Store,
    },
    events,
    game::{
        health::Health,
        missiles::Missile,
        physics::Body,
        ships::Ship,
    },
    world,
};

use super::{
    Explosion,
    ExplosionEntity,
    ExplosionFaded,
    ExplosionImminent,
};


pub fn explode_entity(
    world:    &mut world::Query,
    bodies:   &Store<Body>,
    missiles: &Store<Missile>,
    ships:    &Store<Ship>,
    handle:   hecs::Entity,
)
    -> Option<ExplosionEntity>
{
    let health = world.get::<Health>(handle).ok()?;
    let body   = bodies.get(health.body)?;

    let mut is_missile = false;
    for missile in missiles.values() {
        if handle.to_bits() == missile.entity {
            is_missile = true;
        }
    }

    let mut is_ship = false;
    for ship in ships.values() {
        if handle.to_bits() == ship.entity {
            is_ship = true;
        }
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
    bodies:             &mut Store<Body>,
    explosions:         &mut Store<Explosion>,
    explosion_imminent: &mut events::Sink<ExplosionImminent>,
    explosion:          ExplosionEntity,
) {
    let handle = explosion.create(bodies, explosions);
    explosion_imminent.push(ExplosionImminent { handle });
}

pub fn damage_nearby(
    handle:     Handle,
    world:      &mut world::Query,
    bodies:     &Store<Body>,
    explosions: &Store<Explosion>,
)
    -> Option<()>
{
    let explosion = explosions.get(handle)?;
    let body      = bodies.get(explosion.body)?;

    let query = &mut world.query::<(&mut Health,)>();
    let nearby = query
        .into_iter()
        .filter_map(|(_, (health,))| Some((bodies.get(health.body)?, health)));

    explosion.damage_nearby(&body, nearby);
    Some(())
}

pub fn update_explosions(
    explosions:      &mut Store<Explosion>,
    dt:              f32,
    explosion_faded: &mut events::Sink<ExplosionFaded>,
) {
    for (handle, explosion) in explosions {
        if explosion.update(dt) {
            explosion_faded.push(ExplosionFaded { handle });
        }
    }
}

pub fn remove_explosion(explosions: &mut Store<Explosion>, handle: Handle) {
    explosions.remove(handle);
}
