use std::collections::HashSet;

use toadster::{
    handle,
    store,
};

use crate::world::{
    math::Scalar,
    physics::{
        Body,
        Position,
        Velocity,
    },
};

use super::Explosion;


pub struct ExplosionEntity {
    pub exploding: Body,
    pub strength:  Scalar,
}

impl ExplosionEntity {
    pub fn create(&self,
        explosions: &mut store::Strong<Explosion>,
        positions:  &mut store::Strong<Position>,
        velocities: &mut store::Strong<Velocity>,
        index:      &mut HashSet<handle::Strong<Explosion>>,
    )
        -> Option<handle::Strong<Explosion>>
    {
        let pos = *positions.get(&self.exploding.pos)?;
        let pos = positions.insert(pos);

        let vel = *velocities.get(&self.exploding.vel)?;
        let vel = velocities.insert(Velocity(vel.0 * 0.05));

        let explosion = Explosion::new(pos, vel, self.strength);
        let explosion = explosions.insert(explosion);
        index.insert(explosion.clone());
        Some(explosion)
    }
}
