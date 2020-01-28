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
    for missile in missiles.values_mut() {
        let potential_targets = crafts.values()
            .filter_map(|craft| Some((*bodies.get(craft.body)?, *craft)));

        missile.update_target(crafts, potential_targets);
    }
}

pub fn update_guidance(
    bodies:   &mut Store<Body>,
    crafts:   &Store<Craft>,
    missiles: &mut Store<Missile>,
) {
    for missile in missiles.values_mut() {
        missile.update_guidance(bodies, crafts);
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
