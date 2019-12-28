use crate::game::{
    components::{
        Body,
        Engine,
    },
    entities as e,
};


pub struct Missile;

impl Missile {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, body: &Body, engine: &Engine) -> Option<e::Explosion> {
        if engine.fuel <= 0.0 {
            Some(e::explosion(body))
        }
        else {
            None
        }
    }
}
