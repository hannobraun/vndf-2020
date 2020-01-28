use crate::{
    cgs::Store,
    game::{
        crafts::Craft,
        health::Health,
        physics::Body,
        players::PlayerId,
    },
    world,
};

use super::Ship;


pub struct ShipEntity {
    pub owner: PlayerId,
    pub color: [f32; 3],
}

impl ShipEntity {
    pub fn create(&self,
        world:  &mut world::Spawn,
        bodies: &mut Store<Body>,
        crafts: &mut Store<Craft>,
        ships:  &mut Store<Ship>,
    ) {
        let body   = bodies.insert(Body::new());
        let entity = world.spawn((Health::new(body, 10.0),));

        let craft = Craft {
            body,

            engine_on: false,
            thrust:    100.0,
            fuel:      1200.0,
            owner:     self.owner,
        };
        let craft = crafts.insert(craft);

        ships.insert(Ship::new(entity, craft, self.color));
    }
}
