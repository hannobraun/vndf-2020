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
    for (handle, (health,)) in &mut world.query::<(&Health,)>() {
        if health.is_dead() {
            death.push(Death { handle });
        }
    }
}

pub fn remove_entity(
    world:  &mut world::Spawn,
    handle: hecs::Entity,
)
    -> Option<()>
{
    world.despawn(handle).ok()?;
    Some(())
}
