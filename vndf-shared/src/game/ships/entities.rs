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
    data,
    game::{
        crafts::{
            Craft,
            Fuel,
        },
        health::Health,
        physics::{
            Body,
            Position,
            Velocity,
        },
        planets::{
            G,
            Planet,
        },
        players::PlayerId,
    },
    math::{
        prelude::*,
        Pnt2,
        Rad,
        rotate,
    },
};

use super::Ship;


pub struct ShipEntity {
    pub owner: PlayerId,
    pub color: [f32; 3],
}

impl ShipEntity {
    pub fn create(&self,
        planet:     &Planet,
        bodies:     &mut store::Strong<Body>,
        crafts:     &mut store::Strong<Craft>,
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

        // Compute velocity for circular orbit at the given distance.
        let speed = (G * planet.mass / distance).sqrt();
        let velocity = rotate(
            position.to_vec().normalize() * speed,
            Rad::turn_div_4(),
        );

        let pos    = positions.insert(Position(position));
        let vel    = velocities.insert(Velocity(velocity));
        let body   = bodies.insert(Body::new(pos, vel));
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
            data::client::Handle::Ship(ship.into()),
            entities,
        );
    }
}
