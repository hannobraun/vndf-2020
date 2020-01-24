use crate::{
    cgs::{
        Handle,
        Store,
    },
    events,
    game::{
        explosions::entities::ExplosionEntity,
        health::Health,
        missiles::Missile,
        physics::Body,
        ships::Ship,
    },
    world,
};

use super::{
    components::Explosion,
    events::{
        ExplosionFaded,
        ExplosionImminent,
    }
};


pub fn explode_entity(
    world:    world::Query,
    missiles: &Store<Missile>,
    ships:    &Store<Ship>,
    handle:   hecs::Entity,
)
    -> Option<ExplosionEntity>
{
    let body = world.get::<Body>(handle).ok()?;

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
    world:              &mut world::Spawn,
    explosions:         &mut Store<Explosion>,
    explosion_imminent: &mut events::Sink<ExplosionImminent>,
    explosion:          ExplosionEntity,
) {
    let handle = explosion.create(world, explosions);
    explosion_imminent.push(ExplosionImminent { handle });
}

pub fn damage_nearby(
    world:      &mut world::Query,
    explosions: &Store<Explosion>,
    handle:     Handle,
) {
    let explosion = explosions.get(handle)
        .expect("Explosion not found");
    let body = world.get(hecs::Entity::from_bits(explosion.entity))
        .expect("Explosion not found");

    let query = &mut world.query::<(&Body, &mut Health)>();
    let nearby = query
        .into_iter()
        .map(|(_, (body, health))| (body, health));

    explosion.damage_nearby(&body, nearby);
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
