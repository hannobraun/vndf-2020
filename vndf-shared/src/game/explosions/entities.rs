use crate::{
    game::physics::Body,
    world,
};

use super::components::Explosion;


pub struct ExplosionEntity {
    pub exploding: Body,
    pub strength:  f32,
}

impl ExplosionEntity {
    pub fn create(&self, world: &mut world::Spawn) -> hecs::Entity {
        let body = Body {
            pos: self.exploding.pos,
            vel: self.exploding.vel * 0.05,
            .. Body::new()
        };

        world.spawn((Explosion::new(self.strength), body))
    }
}
