use crate::{
    events,
    game::{
        components::{
            Body,
            Craft,
        },
        entities as e,
        in_event::InEvent,
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

pub fn explode_craft(
    world:     &mut world::Spawn,
    events:    &mut events::Push<InEvent>,
    explosion: e::Explosion,
) {
    let explosion = world.spawn(explosion);
    events.explosion(explosion);
}
