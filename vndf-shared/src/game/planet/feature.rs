use toadster::store;

use crate::math::Pnt2;

use super::Planet;


pub struct Feature {
    pub planets: store::Strong<Planet>,
}

impl Feature {
    pub fn new() -> Self {
        let mut planets = store::Strong::new();

        planets.insert(Planet {
            pos:  Pnt2::new(0.0, 0.0),
            size: 100.0,
        });

        Self {
            planets,
        }
    }
}
