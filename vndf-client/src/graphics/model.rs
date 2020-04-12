use crate::shared::world::{
    self,
    behavior::planets::Planet,
};


pub struct Model {
    pub size:  world::Size,
    pub pos:   world::Pnt2,
    pub angle: world::Angle,
}

impl From<&Planet> for Model {
    fn from(planet: &Planet) -> Self {
        Self {
            size:  world::Size::from_lengths(planet.size, planet.size),
            pos:   planet.pos,
            angle: world::Angle::zero(),
        }
    }
}
