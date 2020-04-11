use serde::{
    Deserialize,
    Serialize,
};

use crate::math::{
    Pnt2,
    Vec2,
};

use super::G;


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

    /// Acceleration of a body at the given position, due to gravity
    pub fn acceleration_at(&self, pos: Pnt2) -> Vec2 {
        let dist = (pos - self.pos).length();
        let acc  = G * self.mass / dist.powi(2);

        (self.pos - pos).normalize() * acc
    }
}
