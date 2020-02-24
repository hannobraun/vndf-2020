pub mod integration;

pub use integration::integrate;


use cgmath::{
    self,
    prelude::*,
};


pub mod prelude {
    pub use cgmath::prelude::*;
}


pub type Pnt2 = cgmath::Point2<f32>;
pub type Vec2 = cgmath::Vector2<f32>;

pub type Rad = cgmath::Rad<f32>;


pub fn rotate(vec: Vec2, angle: Rad) -> Vec2 {
    let rot: cgmath::Basis2<_> = cgmath::Rotation2::from_angle(angle);
    rot.rotate_vector(vec)
}
