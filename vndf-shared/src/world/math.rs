pub mod integration;

pub use integration::integrate;


use euclid::{
    self,
    UnknownUnit,
};


pub type Angle = euclid::Angle<f32>;

pub type Pnt2 = euclid::Point2D<f32, UnknownUnit>;
pub type Vec2 = euclid::Vector2D<f32, UnknownUnit>;


pub fn rotate(vec: Vec2, angle: Angle) -> Vec2 {
    let rot = euclid::Rotation2D::new(angle);
    rot.transform_vector(vec)
}
