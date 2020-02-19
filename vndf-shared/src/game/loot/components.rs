use std::f32;

use serde::{
    Deserialize,
    Serialize,
};
use toadster::{
    Handle,
    handle,
    store,
};

use crate::{
    game::{
        crafts::{
            Craft,
            Fuel,
        },
        health::Health,
        physics::{
            Body,
            Position,
        },
        ships::Ship,
    },
    math::prelude::*,
};


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Loot {
    pub body:     Handle<Body>,
    pub health:   Handle<Health>,
    pub fuel:     f32,
    pub missiles: u64,
}

impl Loot {
    pub fn collect(&self,
        handle:    impl Into<handle::Weak<Loot>>,
        bodies:    &store::Strong<Body>,
        crafts:    &store::Strong<Craft>,
        fuels:     &mut store::Strong<Fuel>,
        loots:     &store::Strong<Loot>,
        positions: &store::Strong<Position>,
        ships:     &mut store::Strong<Ship>,
    ) {
        let mut min_distance = f32::INFINITY;
        let mut nearest_ship = None;

        for (ship_handle, ship) in ships.iter_mut() {
            let distance = self.distance(
                &ship.craft,
                bodies,
                crafts,
                positions,
            );

            if let Some(distance) = distance {
                if distance < min_distance {
                    min_distance = distance;
                    nearest_ship = Some(ship_handle);
                }
            }
        }

        if let Some(ship) = nearest_ship {
            if min_distance < 10.0 {
                self.add_to_ship(&ship, crafts, fuels, ships);
                loots.remove_later(handle);
            }
        }
    }

    fn distance(&self,
        craft:     impl Into<handle::Weak<Craft>>,
        bodies:    &store::Strong<Body>,
        crafts:    &store::Strong<Craft>,
        positions: &store::Strong<Position>,
    )
        -> Option<f32>
    {
        let craft      = crafts.get(craft)?;
        let loot_body  = bodies.get(&self.body)?;
        let other_body = bodies.get(&craft.body)?;
        let loot_pos   = positions.get(&loot_body.pos)?;
        let other_pos  = positions.get(&other_body.pos)?;

        let distance = (loot_pos.0 - other_pos.0).magnitude();

        Some(distance)
    }

    fn add_to_ship(&self,
        ship:   &handle::Strong<Ship>,
        crafts: &store::Strong<Craft>,
        fuels:  &mut store::Strong<Fuel>,
        ships:  &mut store::Strong<Ship>,
    )
        -> Option<()>
    {
        let ship  = ships.get_mut(ship)?;
        let craft = crafts.get(&ship.craft)?;
        let fuel  = fuels.get_mut(&craft.fuel)?;

        fuel.0        += self.fuel;
        ship.missiles += self.missiles;

        Some(())
    }
}
