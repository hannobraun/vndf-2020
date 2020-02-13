use rand::{
    prelude::*,
    random,
    thread_rng,
};
use toadster::{
    StrongHandle,
    StrongStore,
};

use crate::{
    game::{
        WORLD_SIZE,
        base::ComponentHandle,
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
        ships::Ship,
    },
    math::Pnt2,
};

use super::Loot;


pub fn spawn_death_loot(
    handle:     &StrongHandle<Health>,
    bodies:     &mut StrongStore<Body>,
    crafts:     &StrongStore<Craft>,
    directions: &mut StrongStore<Direction>,
    fuels:      &StrongStore<Fuel>,
    healths:    &StrongStore<Health>,
    loots:      &mut StrongStore<Loot>,
    positions:  &mut StrongStore<Position>,
    ships:      &StrongStore<Ship>,
    velocities: &mut StrongStore<Velocity>,
)
    -> Option<()>
{
    let health = healths.get(handle)?;

    if let ComponentHandle::Ship(handle) = health.parent.as_ref()? {
        let ship  = ships.get(handle)?;
        let craft = crafts.get(&ship.craft)?;
        let fuel  = fuels.get(&craft.fuel)?;
        let body  = bodies.get(&craft.body)?;
        let pos   = positions.get(&body.pos)?;
        let vel   = velocities.get(&body.vel)?;
        let dir   = directions.get(&body.dir)?;

        let pos = *pos;
        let pos = positions.insert(pos);

        let vel = *vel;
        let vel = velocities.insert(vel);

        let dir = *dir;
        let dir = directions.insert(dir);

        let body = bodies.insert(Body::new(pos, vel, dir));

        let loot = Loot {
            body,
            fuel:     fuel.0 / 10.0,
            missiles: ship.missiles / 10,
        };

        loots.insert(loot);
    }

    Some(())
}

pub fn spawn_random_loot(
    dt:         f32,
    bodies:     &mut StrongStore<Body>,
    directions: &mut StrongStore<Direction>,
    loots:      &mut StrongStore<Loot>,
    positions:  &mut StrongStore<Position>,
    velocities: &mut StrongStore<Velocity>,
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
    bodies:    &StrongStore<Body>,
    crafts:    &StrongStore<Craft>,
    fuels:     &mut StrongStore<Fuel>,
    loots:     &StrongStore<Loot>,
    positions: &StrongStore<Position>,
    ships:     &mut StrongStore<Ship>,
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
