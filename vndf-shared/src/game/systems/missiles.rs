use crate::{
    game::{
        components::Missile,
        entities as e,
        features::{
            crafts::components::Craft,
            health::components::Health,
            physics::components::Body,
        },
    },
    world,
};


pub fn launch_missile(world: &mut world::Spawn, missile: e::MissileE) {
    world.spawn(missile);
}

pub fn update_missiles(
    world:  world::Query,
) {
    let potential_targets: Vec<_> = (&mut world.query::<(&Body, &Craft)>())
        .into_iter()
        .map(|(_, (&body, &craft))| (body, craft))
        .collect();

    let query = &mut world.query::<
        (&mut Missile, &mut Body, &Craft, &mut Health)
    >();
    for (_, (missile, body, craft, health)) in query {
        missile.update_target(craft, potential_targets.iter().cloned());
        missile.update_guidance(body);

        if missile.should_explode(body, craft) {
            // Setting the missile's health to zero will cause it to explode.
            health.value = 0.0;
        }
    }
}
