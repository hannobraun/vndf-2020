use toadster::StrongStore;

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
    bodies:     &mut StrongStore<Body>,
    crafts:     &mut StrongStore<Craft>,
    directions: &mut StrongStore<Direction>,
    fuels:      &mut StrongStore<Fuel>,
    guidances:  &mut StrongStore<Guidance>,
    healths:    &mut StrongStore<Health>,
    missiles:   &mut StrongStore<Missile>,
    positions:  &mut StrongStore<Position>,
    targets:    &mut StrongStore<Target>,
    velocities: &mut StrongStore<Velocity>,
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
    bodies:    &StrongStore<Body>,
    crafts:    &StrongStore<Craft>,
    positions: &StrongStore<Position>,
    targets:   &mut StrongStore<Target>,
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
    bodies:     &mut StrongStore<Body>,
    crafts:     &StrongStore<Craft>,
    directions: &mut StrongStore<Direction>,
    guidances:  &mut StrongStore<Guidance>,
    positions:  &StrongStore<Position>,
    targets:    &StrongStore<Target>,
    velocities: &StrongStore<Velocity>,
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
    bodies:    &StrongStore<Body>,
    crafts:    &StrongStore<Craft>,
    fuels:     &StrongStore<Fuel>,
    guidances: &StrongStore<Guidance>,
    healths:   &mut StrongStore<Health>,
    positions: &StrongStore<Position>,
    targets:   &StrongStore<Target>,
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
