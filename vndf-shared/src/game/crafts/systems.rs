use crate::{
    cgs::Store,
    game::physics::Body,
    world,
};

use super::Craft;


pub fn update_bodies(world: world::Query, world_size: f32, dt: f32) {
    for (_, (body,)) in &mut world.query::<(&mut Body,)>() {
        body.update(dt);
        body.enforce_boundary(world_size);
    }
}

pub fn update_crafts(
    world:  world::Query,
    crafts: &mut Store<Craft>,
    dt:     f32,
) {
    for craft in crafts.values_mut() {
        let entity = hecs::Entity::from_bits(craft.body);
        if let Ok(mut body) = world.get_mut::<Body>(entity) {
            craft.apply_thrust(&mut body, dt);
        }
    }
}
