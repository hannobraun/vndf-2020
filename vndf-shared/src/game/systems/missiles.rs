use crate::{
    events,
    game::{
        components::{
            Body,
            Craft,
            Explosion,
            Missile,
        },
        entities as e,
        in_event::InEvent,
    },
    world,
};


pub fn launch_missile(world: &mut world::Spawn, missile: e::Missile) {
    world.spawn(missile);
}

pub fn update_missiles(
    world:  world::Query,
    events: &mut events::Push<InEvent>,
) {
    let query = &mut world.query::<(&mut Missile, &mut Body, &Craft)>();
    for (id, (missile, body, craft)) in query {
        missile.update_guidance(body);

        if let Some(explosion) = missile.should_explode(body, craft) {
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
    events: &mut events::Push<InEvent>,
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
