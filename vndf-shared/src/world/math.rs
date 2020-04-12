pub mod integration;

pub use integration::integrate;


use euclid;


pub enum Meter {}


pub type Angle = euclid::Angle<f32>;

pub type Length = euclid::Length<f32, Meter>;
pub type Size   = euclid::Size2D<f32, Meter>;

pub type Pnt2 = euclid::Point2D<f32, Meter>;
pub type Vec2 = euclid::Vector2D<f32, Meter>;


pub fn rotate(vec: Vec2, angle: Angle) -> Vec2 {
    let rot = euclid::Rotation2D::new(angle);
    rot.transform_vector(vec)
}
