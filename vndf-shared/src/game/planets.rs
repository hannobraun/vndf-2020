pub mod components;
pub mod systems;


pub use self::{
    components::*,
    systems::*,
};


/// The gravitational constant
pub const G: f32 = 6.674e-11;
