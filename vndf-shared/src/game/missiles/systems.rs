use crate::{
    cgs::Store,
    game::{
        crafts::Craft,
        health::Health,
        physics::Body,
    },
};

use super::{
    Missile,
    MissileEntity,
};


pub fn launch_missile(
    bodies:   &mut Store<Body>,
    crafts:   &mut Store<Craft>,
    healths:  &mut Store<Health>,
    missiles: &mut Store<Missile>,
    missile:  MissileEntity,
) {
    missile.create(bodies, crafts, healths, missiles);
}

pub fn update_targets(
    bodies:   &Store<Body>,
    crafts:   &Store<Craft>,
    missiles: &mut Store<Missile>,
) {
    for missile in missiles.values_mut() {
        let potential_targets = crafts.values()
            .filter_map(|craft| Some((*bodies.get(craft.body)?, *craft)));

        missile.update_target(crafts, potential_targets);
    }
}

pub fn update_guidances(
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
    healths:  &mut Store<Health>,
    missiles: &Store<Missile>,
) {
    for missile in missiles.values() {
        missile.explode_if_ready(bodies, crafts, healths);
    }
}
