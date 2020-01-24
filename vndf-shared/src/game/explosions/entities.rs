use crate::{
    cgs::{
        Handle,
        Store,
    },
    game::physics::Body,
    world,
};

use super::Explosion;


pub struct ExplosionEntity {
    pub exploding: Body,
    pub strength:  f32,
}

impl ExplosionEntity {
    pub fn create(&self,
        world:      &mut world::Spawn,
        explosions: &mut Store<Explosion>,
    )
        -> Handle
    {
        let body = Body {
            pos: self.exploding.pos,
            vel: self.exploding.vel * 0.05,
            .. Body::new()
        };

        let entity = world.spawn((body,));
        explosions.insert(Explosion::new(entity, self.strength))
    }
}
