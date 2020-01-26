use crate::{
    cgs::Store,
    game::{
        crafts::Craft,
        health::Health,
        missiles::{
            components::Missile,
            entities::MissileEntity,
        },
        physics::Body,
    },
    world,
};


pub fn launch_missile(
    world:    &mut world::Spawn,
    crafts:   &mut Store<Craft>,
    missiles: &mut Store<Missile>,
    missile:  MissileEntity,
) {
    missile.create(world, crafts, missiles);
}

pub fn update_missiles(
    world:    world::Query,
    crafts:   &Store<Craft>,
    missiles: &mut Store<Missile>,
) {
    let potential_targets: Vec<_> = (&mut world.query::<(&Body, &Craft)>())
        .into_iter()
        .map(|(_, (&body, &craft))| (body, craft))
        .collect();

    for (_, missile) in missiles {
        let entity = hecs::Entity::from_bits(missile.entity);

        let body   = world.get_mut::<Body>(entity);
        let craft  = crafts.get(missile.craft);
        let health = world.get_mut::<Health>(entity);

        let (mut body, craft, mut health) = match (body, craft, health) {
            (Ok(body), Some(craft), Ok(health)) => (body, craft, health),
            _                                   => continue,
        };

        missile.update_target(&craft, potential_targets.iter().cloned());
        missile.update_guidance(&mut body);

        if missile.should_explode(&body, &craft) {
            // Setting the missile's health to zero will cause it to explode.
            health.value = 0.0;
        }
    }
}
