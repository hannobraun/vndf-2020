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
    events: &mut events::Push<InEvent>,
    dt:     f32,
) {
    let query = &mut world.query::<(&mut Craft, &mut Body)>();
    for (entity, (craft, body)) in query {
        craft.apply_thrust(body, dt);

        if let Some(explosion) = craft.should_explode(body) {
            events.explode_craft(entity, explosion);
        }
    }
}

pub fn explode_craft(
    world:     &mut world::Spawn,
    craft:     hecs::Entity,
    explosion: e::Explosion,
) {
    world.despawn(craft)
        .expect("Exploding craft not found");
    world.spawn(explosion);
}
