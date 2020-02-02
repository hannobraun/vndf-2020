use crate::{
    cgs::Store,
    game::{
        crafts::Craft,
        health::Health,
        physics::{
            Body,
            Position,
            Velocity,
        },
    },
};

use super::{
    Missile,
    MissileEntity,
};


pub fn launch_missile(
    bodies:     &mut Store<Body>,
    crafts:     &mut Store<Craft>,
    healths:    &mut Store<Health>,
    missiles:   &mut Store<Missile>,
    positions:  &mut Store<Position>,
    velocities: &mut Store<Velocity>,
    missile:   MissileEntity,
) {
    missile.create(bodies, crafts, healths, missiles, positions, velocities);
}

pub fn update_targets(
    bodies:    &Store<Body>,
    crafts:    &Store<Craft>,
    missiles:  &mut Store<Missile>,
    positions: &Store<Position>,
) {
    for missile in missiles.values_mut() {
        let potential_targets = crafts.values()
            .filter_map(|craft| {
                let body = bodies.get(craft.body)?;
                let pos  = positions.get(body.pos)?;
                Some((*pos, *craft))
            });

        missile.update_target(crafts, potential_targets);
    }
}

pub fn update_guidances(
    bodies:     &mut Store<Body>,
    crafts:     &Store<Craft>,
    missiles:   &mut Store<Missile>,
    positions:  &Store<Position>,
    velocities: &Store<Velocity>,
) {
    for missile in missiles.values_mut() {
        missile.update_guidance(bodies, crafts, positions, velocities);
    }
}

pub fn explode_missiles(
    bodies:    &Store<Body>,
    crafts:    &Store<Craft>,
    healths:   &mut Store<Health>,
    missiles:  &Store<Missile>,
    positions: &Store<Position>,
) {
    for missile in missiles.values() {
        missile.explode_if_ready(bodies, crafts, healths, positions);
    }
}
