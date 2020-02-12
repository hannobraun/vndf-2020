use rand::{
    prelude::*,
    random,
    thread_rng,
};

use crate::{
    cgs::Store,
    game::{
        WORLD_SIZE,
        crafts::{
            Craft,
            Fuel,
        },
        physics::{
            Body,
            Direction,
            Position,
            Velocity,
        },
        ships::Ship,
    },
    math::Pnt2,
};

use super::Loot;


pub fn spawn_random_loot(
    dt:         f32,
    bodies:     &mut Store<Body>,
    directions: &mut Store<Direction>,
    loots:      &mut Store<Loot>,
    positions:  &mut Store<Position>,
    velocities: &mut Store<Velocity>,
) {
    const CHANCE_PER_S: f32   = 1.0 / 30.0;
    const MAX_LOOTS:    usize = 10;

    let left_to_spawn = MAX_LOOTS - loots.len();
    let num_loots_mod = left_to_spawn as f32 / MAX_LOOTS as f32;
    let chance        = CHANCE_PER_S * dt * num_loots_mod;

    let r = random::<f32>();
    if r <= chance {
        let pos = Position(
            Pnt2::new(
                thread_rng().gen_range(-WORLD_SIZE / 2.0, WORLD_SIZE / 2.0),
                thread_rng().gen_range(-WORLD_SIZE / 2.0, WORLD_SIZE / 2.0),
            ),
        );

        let pos = positions.insert(pos);
        let vel = velocities.insert(Velocity::new());
        let dir = directions.insert(Direction::new());

        let body = bodies.insert(Body::new(pos, vel, dir));

        let loot = Loot {
            body,
            fuel:     thread_rng().gen_range(15.0, 100.0),
            missiles: thread_rng().gen_range(1, 5),
        };

        loots.insert(loot);
    }
}

pub fn collect_loot(
    bodies:    &Store<Body>,
    crafts:    &Store<Craft>,
    fuels:     &mut Store<Fuel>,
    loots:     &Store<Loot>,
    positions: &Store<Position>,
    ships:     &mut Store<Ship>,
) {
    for (handle, loot) in loots {
        loot.collect(
            handle,
            bodies,
            crafts,
            fuels,
            loots,
            positions,
            ships,
        );
    }
}
