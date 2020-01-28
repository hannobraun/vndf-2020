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
    bodies:   &mut Store<Body>,
    crafts:   &mut Store<Craft>,
    missiles: &mut Store<Missile>,
    missile:  MissileEntity,
) {
    missile.create(world, bodies, crafts, missiles);
}

pub fn update_missiles(
    world:    &mut world::Query,
    bodies:   &mut Store<Body>,
    crafts:   &Store<Craft>,
    missiles: &mut Store<Missile>,
) {
    let potential_targets: Vec<_> = crafts.values()
        .filter_map(|craft| Some((*bodies.get(craft.body)?, *craft)))
        .collect();

    for (_, missile) in missiles {
        let entity = hecs::Entity::from_bits(missile.entity);

        let craft = match crafts.get(missile.craft) {
            Some(craft) => craft,
            None        => continue,
        };

        let body   = bodies.get_mut(craft.body);
        let health = world.get_mut::<Health>(entity);

        let (mut body, mut health) = match (body, health) {
            (Some(body), Ok(health)) => (body, health),
            _                        => continue,
        };

        missile.update_target(&craft, potential_targets.iter().cloned());
        missile.update_guidance(&mut body);
        missile.explode_if_ready(&body, &craft, &mut health);
    }
}
