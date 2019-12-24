use hecs::World;

use crate::state::components::{
    Body,
    Engine,
    Ship,
};


pub fn update_ships(world: &mut World) {
    for (_, (ship, body)) in &mut world.query::<(&mut Ship, &mut Body)>() {
        ship.update(body);
    }
}

pub fn update_engines(world: &mut World) {
    for (_, (engine, body)) in &mut world.query::<(&Engine, &mut Body)>() {
        engine.update(body)
    }
}

pub fn update_bodies(world: &mut World, world_size: f32, dt: f32) {
    for (_, (body,)) in &mut world.query::<(&mut Body,)>() {
        body.update(dt);
        body.enforce_boundary(world_size);
    }
}
