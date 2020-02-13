use toadster::StrongHandle;

use super::Explosion;


pub struct ExplosionImminent {
    pub handle: StrongHandle<Explosion>,
}

pub struct ExplosionFaded {
    pub handle: StrongHandle<Explosion>,
}
