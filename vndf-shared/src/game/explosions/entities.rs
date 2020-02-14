use toadster::{
    StrongHandle,
    store,
};

use crate::game::physics::{
    Body,
    Direction,
    Position,
    Velocity,
};

use super::Explosion;


pub struct ExplosionEntity {
    pub exploding: Body,
    pub strength:  f32,
}

impl ExplosionEntity {
    pub fn create(&self,
        bodies:     &mut store::Strong<Body>,
        directions: &mut store::Strong<Direction>,
        explosions: &mut store::Strong<Explosion>,
        positions:  &mut store::Strong<Position>,
        velocities: &mut store::Strong<Velocity>,
    )
        -> Option<StrongHandle<Explosion>>
    {
        let pos = *positions.get(&self.exploding.pos)?;
        let pos = positions.insert(pos);

        let vel = *velocities.get(&self.exploding.vel)?;
        let vel = velocities.insert(Velocity(vel.0 * 0.05));

        let dir = directions.insert(Direction::new());

        let body = Body::new(pos, vel, dir);
        let body = bodies.insert(body);

        let explosion = explosions.insert(Explosion::new(body, self.strength));
        Some(explosion)
    }
}
