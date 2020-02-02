use crate::{
    cgs::Store,
    game::{
        base::ComponentHandle,
        crafts::Craft,
        health::Health,
        physics::{
            Body,
            Position,
            Velocity,
        },
        players::PlayerId,
    },
};

use super::Ship;


pub struct ShipEntity {
    pub owner: PlayerId,
    pub color: [f32; 3],
}

impl ShipEntity {
    pub fn create(&self,
        bodies:     &mut Store<Body>,
        crafts:     &mut Store<Craft>,
        healths:    &mut Store<Health>,
        positions:  &mut Store<Position>,
        ships:      &mut Store<Ship>,
        velocities: &mut Store<Velocity>,
    ) {
        let pos    = positions.insert(Position::new());
        let vel    = velocities.insert(Velocity::new());
        let body   = bodies.insert(Body::new(pos, vel));
        let health = healths.insert(Health::new(body, 10.0));

        let craft = Craft {
            body,
            health,

            engine_on: false,
            thrust:    100.0,
            fuel:      1200.0,
            owner:     self.owner,
        };
        let craft = crafts.insert(craft);

        let ship = ships.insert(Ship::new(craft, self.color));
        healths.get_mut(health).unwrap().parent =
            Some(ComponentHandle::Ship(ship));
    }
}
