use crate::{
    game::{
        components::{
            Body,
            Engine,
            Explosion,
            Missile,
            Ship,
        },
        events,
    },
    world,
};


pub fn update_ships(world: &mut world::Query) {
    for (_, (ship, body)) in &mut world.query::<(&mut Ship, &mut Body)>() {
        ship.update(body);
    }
}

pub fn update_engines(world: &mut world::Query, dt: f32) {
    for (_, (engine, body)) in &mut world.query::<(&mut Engine, &mut Body)>() {
        engine.update(body, dt)
    }
}

pub fn update_bodies(world: &mut world::Query, world_size: f32, dt: f32) {
    for (_, (body,)) in &mut world.query::<(&mut Body,)>() {
        body.update(dt);
        body.enforce_boundary(world_size);
    }
}

pub fn update_missiles(world: &mut world::Query, events: &mut events::Push) {
    let query = &mut world.query::<(&Missile, &Body, &Engine)>();
    for (id, (missile, body, engine)) in query {
        if let Some(explosion) = missile.update(body, engine) {
            events.explode_missile(id, explosion);
        }
    }
}

pub fn update_explosions(
    world:  &mut world::Query,
    dt:     f32,
    events: &mut events::Push,
) {
    for (id, (explosion,)) in &mut world.query::<(&mut Explosion,)>() {
        if explosion.update(dt) {
            events.remove_explosion(id);
        }
    }
}
