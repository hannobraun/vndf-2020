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
    pub mass: f32,
}

impl Planet {
    pub fn to_weak(&self) -> Self {
        self.clone()
    }

    pub fn gravitation_at(&self, pos: Pnt2) -> Vec2 {
        // The gravitational constant of our universe. Completely made up.
        const G: f32 = 5.0;

        let dist = pos.distance(self.pos);
        let acc  = G * self.mass / dist.powi(2);

        (self.pos - pos).normalize() * acc
    }
}
