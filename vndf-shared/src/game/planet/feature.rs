use toadster::store;

use super::Planet;


pub struct Feature {
    pub planets: store::Strong<Planet>,
}

impl Feature {
    pub fn new() -> Self {
        let mut planets = store::Strong::new();

        planets.insert(Planet { size: 100.0 });

        Self {
            planets,
        }
    }
}
