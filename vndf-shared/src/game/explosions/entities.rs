use crate::{
    cgs::{
        Handle,
        Store,
    },
    game::physics::{
        Body,
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
        explosions: &mut Store<Explosion>,
        positions:  &mut Store<Position>,
        velocities: &mut Store<Velocity>,
    )
        -> Option<Handle>
    {
        let pos = *positions.get(self.exploding.pos)?;
        let pos = positions.insert(pos);

        let vel = *velocities.get(self.exploding.vel)?;
        let vel = velocities.insert(Velocity(vel.0 * 0.05));

        let body = Body::new(pos, vel);
        let body = bodies.insert(body);

        let explosion = explosions.insert(Explosion::new(body, self.strength));
        Some(explosion)
    }
}
