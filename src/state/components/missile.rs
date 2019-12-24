use crate::state::{
    components::{
        Body,
        Engine,
        Explosion,
    },
    entities,
};


pub struct Missile;

impl Missile {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, body: &Body, engine: &Engine)
        -> Option<(Explosion, Body)>
    {
        if engine.fuel <= 0.0 {
            Some(entities::explosion(body))
        }
        else {
            None
        }
    }
}
