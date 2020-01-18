use crate::{
    events,
    game::{
        components::Body,
        entities as e,
        in_event::InEvent,
    },
    world,
};


pub fn explode_entity(
    world:  world::Query,
    entity: hecs::Entity,
)
    -> Option<e::Explosion>
{
    let body = world.get::<Body>(entity).ok()?;
    Some(e::explosion(&body))
}

pub fn create_explosion(
    world:     &mut world::Spawn,
    events:    &mut events::Push<InEvent>,
    explosion: e::Explosion,
) {
    let explosion = world.spawn(explosion);
    events.explosion(explosion);
}
