use crate::{
    cgs::{
        Handle,
        Store,
    },
    game::physics::Body,
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
    )
        -> Option<Handle>
    {
        let body = Body {
            pos: self.exploding.pos,
            vel: self.exploding.vel * 0.05,
            .. Body::new()
        };
        let body = bodies.insert(body);

        let explosion = explosions.insert(Explosion::new(body, self.strength));
        Some(explosion)
    }
}
