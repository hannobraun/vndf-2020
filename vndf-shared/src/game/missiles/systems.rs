use toadster::store;

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
    bodies:     &mut store::Strong<Body>,
    crafts:     &mut store::Strong<Craft>,
    directions: &mut store::Strong<Direction>,
    fuels:      &mut store::Strong<Fuel>,
    guidances:  &mut store::Strong<Guidance>,
    healths:    &mut store::Strong<Health>,
    missiles:   &mut store::Strong<Missile>,
    positions:  &mut store::Strong<Position>,
    targets:    &mut store::Strong<Target>,
    velocities: &mut store::Strong<Velocity>,
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
    bodies:    &store::Strong<Body>,
    crafts:    &store::Strong<Craft>,
    positions: &store::Strong<Position>,
    targets:   &mut store::Strong<Target>,
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
    bodies:     &mut store::Strong<Body>,
    crafts:     &store::Strong<Craft>,
    directions: &mut store::Strong<Direction>,
    guidances:  &mut store::Strong<Guidance>,
    positions:  &store::Strong<Position>,
    targets:    &store::Strong<Target>,
    velocities: &store::Strong<Velocity>,
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
    bodies:    &store::Strong<Body>,
    crafts:    &store::Strong<Craft>,
    fuels:     &store::Strong<Fuel>,
    guidances: &store::Strong<Guidance>,
    healths:   &mut store::Strong<Health>,
    positions: &store::Strong<Position>,
    targets:   &store::Strong<Target>,
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
