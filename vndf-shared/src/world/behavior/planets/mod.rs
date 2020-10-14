pub mod components;
pub mod systems;


pub use self::{
    components::*,
    systems::*,
};


use crate::world::math::Scalar;


/// The gravitational constant
pub const G: Scalar = 6.674e-11;
