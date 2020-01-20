use crate::{
    events,
    game::in_event::InEvent,
    world,
};

use super::components::Health;


pub fn check_health(
    world:  world::Query,
    events: &mut events::Push<InEvent>,
) {
    for (entity, (health,)) in &mut world.query::<(&Health,)>() {
        if health.is_dead() {
            events.death(entity);
        }
    }
}

pub fn remove_entity(
    world:  &mut world::Spawn,
    entity: hecs::Entity,
)
    -> Option<()>
{
    world.despawn(entity).ok()?;
    Some(())
}
