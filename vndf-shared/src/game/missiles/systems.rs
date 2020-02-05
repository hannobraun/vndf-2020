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
    Target,
};


pub fn launch_missile(
    bodies:     &mut Store<Body>,
    crafts:     &mut Store<Craft>,
    healths:    &mut Store<Health>,
    missiles:   &mut Store<Missile>,
    positions:  &mut Store<Position>,
    targets:    &mut Store<Target>,
    velocities: &mut Store<Velocity>,
    missile:    MissileEntity,
) {
    missile.create(
        bodies,
        crafts,
        healths,
        missiles,
        positions,
        targets,
        velocities,
    );
}

pub fn update_targets(
    bodies:    &Store<Body>,
    crafts:    &Store<Craft>,
    positions: &Store<Position>,
    targets:   &mut Store<Target>,
) {
    for target in targets.values_mut() {
        let potential_targets = crafts.values()
            .filter_map(|craft| {
                let body = bodies.get(craft.body)?;
                let pos  = positions.get(body.pos)?;
                Some((*pos, *craft))
            });

        target.update_target(crafts, potential_targets);
    }
}

pub fn update_guidances(
    bodies:     &mut Store<Body>,
    crafts:     &Store<Craft>,
    missiles:   &mut Store<Missile>,
    positions:  &Store<Position>,
    targets:    &Store<Target>,
    velocities: &Store<Velocity>,
) {
    for missile in missiles.values_mut() {
        missile.update_guidance(
            bodies,
            crafts,
            positions,
            targets,
            velocities,
        );
    }
}

pub fn explode_missiles(
    bodies:    &Store<Body>,
    crafts:    &Store<Craft>,
    healths:   &mut Store<Health>,
    missiles:  &Store<Missile>,
    positions: &Store<Position>,
    targets:   &Store<Target>,
) {
    for missile in missiles.values() {
        missile.explode_if_ready(
            bodies,
            crafts,
            healths,
            positions,
            targets,
        );
    }
}
