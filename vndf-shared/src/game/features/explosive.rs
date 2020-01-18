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
    events: &mut events::Push<InEvent>,
    entity: hecs::Entity,
)
    -> Option<()>
{
    let body = world.get::<Body>(entity).ok()?;
    events.explode_craft(entity, e::explosion(&body));
    Some(())
}

pub fn create_explosion(
    world:     &mut world::Spawn,
    events:    &mut events::Push<InEvent>,
    explosion: e::Explosion,
) {
    let explosion = world.spawn(explosion);
    events.explosion(explosion);
}
