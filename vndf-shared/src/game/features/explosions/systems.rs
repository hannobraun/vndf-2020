use crate::{
    events,
    game::{
        entities as e,
        features::{
            health::components::Health,
            missiles::components::Missile,
            physics::components::Body,
            ships::components::Ship,
        },
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
    entity: hecs::Entity,
)
    -> Option<e::ExplosionE>
{
    let body    = world.get::<Body>(entity).ok()?;
    let missile = world.get::<Missile>(entity).ok();
    let ship    = world.get::<Ship>(entity).ok();

    let mut strength = 0.0;
    if missile.is_some() {
        strength += 3.0;
    }
    if ship.is_some() {
        strength += 6.0;
    }

    Some(e::explosion(&body, strength))
}

pub fn create_explosion(
    world:              &mut world::Spawn,
    explosion_imminent: &mut events::Sink<ExplosionImminent>,
    explosion:          e::ExplosionE,
) {
    let explosion = world.spawn(explosion);
    explosion_imminent.push(ExplosionImminent { explosion });
}

pub fn damage_nearby(
    world:  &mut world::Query,
    entity: hecs::Entity,
) {
    let explosion = world.get::<Explosion>(entity)
        .expect("Explosion not found");
    let body = world.get(entity)
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
    for (entity, (explosion,)) in &mut world.query::<(&mut Explosion,)>() {
        if explosion.update(dt) {
            explosion_faded.push(ExplosionFaded { entity });
        }
    }
}

pub fn remove_explosion(world: &mut world::Spawn, explosion: hecs::Entity) {
    world.despawn(explosion)
        .expect("Explosion should exist");
}
