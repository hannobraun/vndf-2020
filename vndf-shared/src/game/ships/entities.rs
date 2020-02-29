use std::collections::HashSet;

use rand::{
    prelude::*,
    thread_rng,
};
use toadster::{
    handle::{
        self,
        Untyped,
    },
    store,
};

use crate::{
    data::ClientHandle,
    game::{
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
        players::PlayerId,
    },
    math::{
        prelude::*,
        Pnt2,
        Rad,
    },
};

use super::Ship;


pub struct ShipEntity {
    pub owner: PlayerId,
    pub color: [f32; 3],
}

impl ShipEntity {
    pub fn create(&self,
        bodies:     &mut store::Strong<Body>,
        crafts:     &mut store::Strong<Craft>,
        directions: &mut store::Strong<Direction>,
        fuels:      &mut store::Strong<Fuel>,
        healths:    &mut store::Strong<Health>,
        positions:  &mut store::Strong<Position>,
        ships:      &mut store::Strong<Ship>,
        velocities: &mut store::Strong<Velocity>,
        entities:   &mut HashSet<handle::Strong<Untyped>>,
    ) {
        const THRUST: f32 =    2.5;
        const FUEL:   f32 = 1200.0;
        const HEALTH: f32 =   10.0;

        let distance = 400.0;
        let angle = cgmath::Rad(
            thread_rng().gen_range(0.0, Rad::full_turn().0),
        );
        let position = Pnt2::new(
            angle.sin() * distance,
            angle.cos() * distance,
        );

        let pos    = positions.insert(Position(position));
        let vel    = velocities.insert(Velocity::new());
        let dir    = directions.insert(Direction::new());
        let body   = bodies.insert(Body::new(pos, vel, dir));
        let fuel   = fuels.insert(Fuel(FUEL));
        let health = healths.insert(Health::new(body.clone(), HEALTH));

        let craft = Craft {
            body:   body.into(),
            fuel:   fuel.into(),
            health: health.clone().into(),

            engine_on: false,
            thrust:    THRUST,
            owner:     self.owner,
        };
        let craft = crafts.insert(craft);

        let ship = ships.insert(Ship::new(craft, self.color));
        healths.get_mut(&health).unwrap().finalize(
            ClientHandle::Ship(ship.into()),
            entities,
        );
    }
}
