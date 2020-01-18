use crate::{
    events,
    game::{
        components::{
            Body,
            Craft,
            Missile,
        },
        entities as e,
        in_event::InEvent,
    },
    world,
};


pub fn launch_missile(world: &mut world::Spawn, missile: e::Missile) {
    world.spawn(missile);
}

pub fn update_missiles(
    world:  world::Query,
    events: &mut events::Push<InEvent>,
) {
    let potential_targets: Vec<_> = (&mut world.query::<(&Body, &Craft)>())
        .into_iter()
        .map(|(_, (&body, &craft))| (body, craft))
        .collect();

    let query = &mut world.query::<(&mut Missile, &mut Body, &Craft)>();
    for (id, (missile, body, craft)) in query {
        missile.update_target(craft, potential_targets.iter().cloned());
        missile.update_guidance(body);

        if let Some(explosion) = missile.should_explode(body, craft) {
            events.explode_craft(id, explosion);
        }
    }
}
