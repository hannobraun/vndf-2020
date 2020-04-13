use euclid;


pub enum ClipUnit {}
pub enum LocalUnit {}
pub enum Pixel {}


pub type Angle = euclid::Angle<f32>;
pub type Pnt2  = euclid::Point2D<f32, Pixel>;
pub type Vec2  = euclid::Vector2D<f32, Pixel>;
pub type Size  = euclid::Size2D<f32, Pixel>;
