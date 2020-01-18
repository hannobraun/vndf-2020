use crate::{
    game::components::{
        Body,
        Craft,
    },
    world,
};


pub fn update_bodies(world: world::Query, world_size: f32, dt: f32) {
    for (_, (body,)) in &mut world.query::<(&mut Body,)>() {
        body.update(dt);
        body.enforce_boundary(world_size);
    }
}

pub fn update_crafts(world: world::Query, dt: f32) {
    for (_, (craft, body)) in &mut world.query::<(&mut Craft, &mut Body)>() {
        craft.apply_thrust(body, dt)
    }
}
