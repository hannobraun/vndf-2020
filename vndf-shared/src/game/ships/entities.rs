use toadster::store;

use crate::game::{
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
    players::PlayerId,
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
    ) {
        const THRUST: f32 =    2.5;
        const FUEL:   f32 = 1200.0;
        const HEALTH: f32 =   10.0;

        let pos    = positions.insert(Position::new());
        let vel    = velocities.insert(Velocity::new());
        let dir    = directions.insert(Direction::new());
        let body   = bodies.insert(Body::new(pos, vel, dir));
        let fuel   = fuels.insert(Fuel(FUEL));
        let health = healths.insert(Health::new(body.clone(), HEALTH));

        let craft = Craft {
            body,
            fuel,
            health: health.clone(),

            engine_on: false,
            thrust:    THRUST,
            owner:     self.owner,
        };
        let craft = crafts.insert(craft);

        let ship = ships.insert(Ship::new(craft, self.color));
        healths.get_mut(&health).unwrap().parent =
            Some(ComponentHandle::Ship(ship));
    }
}
