use crate::{
    events,
    game::{
        components::{
            Body,
            Explosion,
        },
        features::health::components::Health,
        in_event::InEvent,
    },
    world,
};


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
    world:  world::Query,
    dt:     f32,
    events: &mut events::Push<InEvent>,
) {
    for (entity, (explosion,)) in &mut world.query::<(&mut Explosion,)>() {
        if explosion.update(dt) {
            events.explosion_faded(entity);
        }
    }
}

pub fn remove_explosion(world: &mut world::Spawn, explosion: hecs::Entity) {
    world.despawn(explosion)
        .expect("Explosion should exist");
}
