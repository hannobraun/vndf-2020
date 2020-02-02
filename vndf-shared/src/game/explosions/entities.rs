use crate::{
    cgs::{
        Handle,
        Store,
    },
    game::physics::{
        Body,
        Position,
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
    )
        -> Option<Handle>
    {
        let pos = *positions.get(self.exploding.pos)?;
        let pos = positions.insert(pos);

        let body = Body {
            vel: self.exploding.vel * 0.05,
            .. Body::new(pos)
        };
        let body = bodies.insert(body);

        let explosion = explosions.insert(Explosion::new(body, self.strength));
        Some(explosion)
    }
}
