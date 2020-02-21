use std::f32::consts::PI;

use serde::{
    Deserialize,
    Serialize,
};
use toadster::store::Store;

use crate::{
    game::physics::{
        Body,
        Position,
    },
    math::{
        prelude::*,
        Pnt2,
    },
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

    pub fn apply_gravitation(&self,
        body:      &mut Body,
        positions: &impl Store<Position>,
    )
        -> Option<()>
    {
        let pos = positions.get(&body.pos)?;

        // The gravitational constant of our universe. Completely made up.
        const G: f32 = 5.0;

        let dist = pos.0.distance(self.pos);
        let mass = PI * self.size.powi(2);
        let acc  = G * mass / dist.powi(2);

        let acc = (self.pos - pos.0).normalize() * acc;
        body.acc += acc;

        Some(())
    }
}
