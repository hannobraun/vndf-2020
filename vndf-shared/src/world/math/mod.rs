pub mod integration;

pub use integration::integrate;


use euclid;


pub enum Meter {}


pub type Scalar = f64;

pub type Angle = euclid::Angle<Scalar>;

pub type Length = euclid::Length<Scalar, Meter>;
pub type Size   = euclid::Size2D<Scalar, Meter>;

pub type Pnt2 = euclid::Point2D<Scalar, Meter>;
pub type Vec2 = euclid::Vector2D<Scalar, Meter>;


pub fn rotate(vec: Vec2, angle: Angle) -> Vec2 {
    let rot = euclid::Rotation2D::new(angle);
    rot.transform_vector(vec)
}
