use crate::{
    game::{
        components::{
            Body,
            Engine,
            Explosion,
            Missile,
        },
        entities as e,
        events,
    },
    world,
};


pub fn launch_missile(world: &mut world::Spawn, missile: e::Missile) {
    world.spawn(missile);
}

pub fn update_missiles(world: world::Query, events: &mut events::Push) {
    let query = &mut world.query::<(&Missile, &Body, &Engine)>();
    for (id, (missile, body, engine)) in query {
        if let Some(explosion) = missile.update(body, engine) {
            events.explode_missile(id, explosion);
        }
    }
}

pub fn explode_missile(
    world:     &mut world::Spawn,
    missile:   hecs::Entity,
    explosion: e::Explosion,
) {
    world.despawn(missile)
        .expect("Missile should exist");
    world.spawn(explosion);
}

pub fn update_explosions(
    world:  world::Query,
    dt:     f32,
    events: &mut events::Push,
) {
    for (id, (explosion,)) in &mut world.query::<(&mut Explosion,)>() {
        if explosion.update(dt) {
            events.remove_explosion(id);
        }
    }
}

pub fn remove_explosion(world: &mut world::Spawn, explosion: hecs::Entity) {
    world.despawn(explosion)
        .expect("Explosion should exist");
}
