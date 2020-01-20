use crate::{
    events,
    game::{
        components::{
            Body,
            Missile,
            Ship,
        },
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
    let body    = world.get::<Body>(entity).ok()?;
    let missile = world.get::<Missile>(entity).ok();
    let ship    = world.get::<Ship>(entity).ok();

    let mut strength = 0.0;
    if missile.is_some() {
        strength += 3.0;
    }
    if ship.is_some() {
        strength += 6.0;
    }

    Some(e::explosion(&body, strength))
}

pub fn create_explosion(
    world:     &mut world::Spawn,
    events:    &mut events::Push<InEvent>,
    explosion: e::Explosion,
) {
    let explosion = world.spawn(explosion);
    events.explosion(explosion);
}
