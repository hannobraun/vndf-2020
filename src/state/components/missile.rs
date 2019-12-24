use crate::state::components::Engine;


pub struct Missile;

impl Missile {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, engine: &Engine) -> bool {
        engine.fuel <= 0.0
    }
}
