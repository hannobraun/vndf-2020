use toadster::handle;

use super::Explosion;


pub struct ExplosionImminent {
    pub handle: handle::Strong<Explosion>,
}

pub struct ExplosionFaded {
    pub handle: handle::Strong<Explosion>,
}
