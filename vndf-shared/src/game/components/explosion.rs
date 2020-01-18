use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    game::components::{
        Body,
        Craft,
    },
    math::prelude::*,
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Explosion {
    pub time_total: f32,
    pub time_left:  f32,
}

impl Explosion {
    pub fn new() -> Self {
        let time_total = 3.0;

        Self {
            time_total,
            time_left: time_total,
        }
    }

    pub fn damage_nearby_crafts<'r>(&self,
        body:   &Body,
        nearby: impl IntoIterator<Item=(&'r Body, &'r mut Craft)>,
    ) {
        for (nearby_body, nearby_craft) in nearby {
            let distance  = (nearby_body.pos - body.pos).magnitude();

            if distance > 20.0 {
                continue;
            }

            let damage = f32::min(1.0 / distance, 5.0);
            nearby_craft.health -= damage;
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        if self.time_left > 0.0 {
            self.time_left -= dt;
            false
        }
        else {
            true
        }
    }
}
