use crate::{
    cgs::Store,
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
    world:  world::Query,
    ships:  &Store<Ship>,
    entity: hecs::Entity,
)
    -> Option<ExplosionEntity>
{
    let body    = world.get::<Body>(entity).ok()?;
    let missile = world.get::<Missile>(entity).ok();

    let mut is_ship = false;
    for ship in ships.values() {
        if entity.to_bits() == ship.entity {
            is_ship = true;
        }
    }

    let mut strength = 0.0;
    if missile.is_some() {
        strength += 3.0;
    }
    if is_ship {
        strength += 6.0;
    }

    Some(ExplosionEntity { exploding: *body, strength })
}

pub fn create_explosion(
    world:              &mut world::Spawn,
    explosion_imminent: &mut events::Sink<ExplosionImminent>,
    explosion:          ExplosionEntity,
) {
    let handle = explosion.create(world);
    explosion_imminent.push(ExplosionImminent { handle });
}

pub fn damage_nearby(
    world:  &mut world::Query,
    handle: hecs::Entity,
) {
    let explosion = world.get::<Explosion>(handle)
        .expect("Explosion not found");
    let body = world.get(handle)
        .expect("Explosion not found");

    let query = &mut world.query::<(&Body, &mut Health)>();
    let nearby = query
        .into_iter()
        .map(|(_, (body, health))| (body, health));

    explosion.damage_nearby(&body, nearby);
}

pub fn update_explosions(
    world:           world::Query,
    dt:              f32,
    explosion_faded: &mut events::Sink<ExplosionFaded>,
) {
    for (handle, (explosion,)) in &mut world.query::<(&mut Explosion,)>() {
        if explosion.update(dt) {
            explosion_faded.push(ExplosionFaded { handle });
        }
    }
}

pub fn remove_explosion(world: &mut world::Spawn, explosion: hecs::Entity) {
    world.despawn(explosion)
        .expect("Explosion should exist");
}
