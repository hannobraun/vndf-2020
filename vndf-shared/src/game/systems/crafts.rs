use crate::{
    game::components::{
        Body,
        Engine,
    },
    world,
};


pub fn update_bodies(world: world::Query, world_size: f32, dt: f32) {
    for (_, (body,)) in &mut world.query::<(&mut Body,)>() {
        body.update(dt);
        body.enforce_boundary(world_size);
    }
}

pub fn update_engines(world: world::Query, dt: f32) {
    for (_, (engine, body)) in &mut world.query::<(&mut Engine, &mut Body)>() {
        engine.update(body, dt)
    }
}
