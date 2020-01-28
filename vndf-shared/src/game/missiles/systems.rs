use crate::{
    cgs::Store,
    game::{
        crafts::Craft,
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
    bodies:   &mut Store<Body>,
    crafts:   &Store<Craft>,
    missiles: &mut Store<Missile>,
) {
    let potential_targets: Vec<_> = crafts.values()
        .filter_map(|craft| Some((*bodies.get(craft.body)?, *craft)))
        .collect();

    for (_, missile) in missiles {
        let craft = match crafts.get(missile.craft) {
            Some(craft) => craft,
            None        => continue,
        };

        let body = bodies.get_mut(craft.body);

        let mut body = match body {
            Some(body) => body,
            _          => continue,
        };

        missile.update_target(&craft, potential_targets.iter().cloned());
        missile.update_guidance(&mut body);
    }
}

pub fn explode_missiles(
    bodies:   &Store<Body>,
    crafts:   &Store<Craft>,
    missiles: &Store<Missile>,
    world:    &mut world::Query,
) {
    for missile in missiles.values() {
        missile.explode_if_ready(bodies, crafts, world);
    }
}
