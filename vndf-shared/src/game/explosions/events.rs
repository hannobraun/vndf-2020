use toadster::Handle;

use super::Explosion;


pub struct ExplosionImminent {
    pub handle: Handle<Explosion>,
}

pub struct ExplosionFaded {
    pub handle: Handle<Explosion>,
}
