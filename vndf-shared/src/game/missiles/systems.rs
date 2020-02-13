use toadster::Store;

use crate::game::{
    crafts::{
        Craft,
        Fuel,
    },
    health::Health,
    physics::{
        Body,
        Direction,
        Position,
        Velocity,
    },
};

use super::{
    Guidance,
    Missile,
    MissileEntity,
    Target,
};


pub fn launch_missile(
    bodies:     &mut Store<Body>,
    crafts:     &mut Store<Craft>,
    directions: &mut Store<Direction>,
    fuels:      &mut Store<Fuel>,
    guidances:  &mut Store<Guidance>,
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
        directions,
        fuels,
        guidances,
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
                let body = bodies.get(&craft.body)?;
                let pos  = positions.get(&body.pos)?;
                Some((*pos, *craft))
            });

        target.update(crafts, potential_targets);
    }
}

pub fn update_guidances(
    bodies:     &mut Store<Body>,
    crafts:     &Store<Craft>,
    directions: &mut Store<Direction>,
    guidances:  &mut Store<Guidance>,
    positions:  &Store<Position>,
    targets:    &Store<Target>,
    velocities: &Store<Velocity>,
) {
    for guidance in guidances.values_mut() {
        guidance.update_guidance(
            bodies,
            crafts,
            directions,
            positions,
            targets,
            velocities,
        );
    }
}

pub fn explode_missiles(
    bodies:    &Store<Body>,
    crafts:    &Store<Craft>,
    fuels:     &Store<Fuel>,
    guidances: &Store<Guidance>,
    healths:   &mut Store<Health>,
    positions: &Store<Position>,
    targets:   &Store<Target>,
) {
    for guidance in guidances.values() {
        guidance.explode_if_ready(
            bodies,
            crafts,
            fuels,
            healths,
            positions,
            targets,
        );
    }
}
