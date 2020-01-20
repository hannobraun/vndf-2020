use crate::{
    events,
    world,
};

use super::{
    components::Health,
    events::Death,
};


pub fn check_health(
    world: world::Query,
    death: &mut events::Sink<Death>,
) {
    for (entity, (health,)) in &mut world.query::<(&Health,)>() {
        if health.is_dead() {
            death.push(Death { entity });
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
