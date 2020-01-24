use crate::{
    game::{
        crafts::components::Craft,
        physics::components::Body,
    },
    world,
};


pub fn update_bodies(world: world::Query, world_size: f32, dt: f32) {
    for (_, (body,)) in &mut world.query::<(&mut Body,)>() {
        body.update(dt);
        body.enforce_boundary(world_size);
    }
}

pub fn update_crafts(
    world:  world::Query,
    dt:     f32,
) {
    let query = &mut world.query::<(&mut Craft, &mut Body)>();
    for (_, (craft, body)) in query {
        craft.apply_thrust(body, dt);
    }
}
