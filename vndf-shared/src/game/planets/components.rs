use std::f32::consts::PI;

use serde::{
    Deserialize,
    Serialize,
};

use crate::math::{
    prelude::*,
    Pnt2,
    Vec2,
};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Planet {
    pub pos:  Pnt2,
    pub size: f32,
}

impl Planet {
    pub fn to_weak(&self) -> Self {
        self.clone()
    }

    pub fn gravitation_at(&self, pos: Pnt2) -> Vec2 {
        // The gravitational constant of our universe. Completely made up.
        const G: f32 = 5.0;

        let dist = pos.distance(self.pos);
        let mass = PI * self.size.powi(2);
        let acc  = G * mass / dist.powi(2);

        (self.pos - pos).normalize() * acc
    }
}
