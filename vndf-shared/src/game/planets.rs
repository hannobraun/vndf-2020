pub mod components;
pub mod systems;


pub use self::{
    components::*,
    systems::*,
};


/// The gravitational constant of our universe. Completely made up.
pub const G: f32 = 5.0;
