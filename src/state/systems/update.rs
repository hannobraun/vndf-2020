use hecs::World;

use crate::state::components::{
    Body,
    Engine,
    Missile,
    Ship,
};


pub fn update_ships(world: &mut World) {
    for (_, (ship, body)) in &mut world.query::<(&mut Ship, &mut Body)>() {
        ship.update(body);
    }
}

pub fn update_engines(world: &mut World, dt: f32) {
    for (_, (engine, body)) in &mut world.query::<(&mut Engine, &mut Body)>() {
        engine.update(body, dt)
    }
}

pub fn update_bodies(world: &mut World, world_size: f32, dt: f32) {
    for (_, (body,)) in &mut world.query::<(&mut Body,)>() {
        body.update(dt);
        body.enforce_boundary(world_size);
    }
}

pub fn update_missiles(world: &mut World) {
    let mut explode = Vec::new();

    for (id, (missile, engine)) in &mut world.query::<(&Missile, &Engine)>() {
        if missile.update(engine) {
            explode.push(id);
        }
    }

    for id in explode {
        world.despawn(id)
            .expect("Tried to explode missile that doesn't exist");
    }
}
