use crate::{
    cgs::{
        Handle,
        Store,
    },
    game::physics::{
        Body,
        Direction,
        Position,
        Velocity,
    },
};

use super::Explosion;


pub struct ExplosionEntity {
    pub exploding: Body,
    pub strength:  f32,
}

impl ExplosionEntity {
    pub fn create(&self,
        bodies:     &mut Store<Body>,
        directions: &mut Store<Direction>,
        explosions: &mut Store<Explosion>,
        positions:  &mut Store<Position>,
        velocities: &mut Store<Velocity>,
    )
        -> Option<Handle<Explosion>>
    {
        let pos = *positions.get(self.exploding.pos)?;
        let pos = positions.insert(pos);

        let vel = *velocities.get(self.exploding.vel)?;
        let vel = velocities.insert(Velocity(vel.0 * 0.05));

        let dir = directions.insert(Direction::new());

        let body = Body::new(pos, vel, dir);
        let body = bodies.insert(body);

        let explosion = explosions.insert(Explosion::new(body, self.strength));
        Some(explosion)
    }
}
