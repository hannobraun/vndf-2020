use euclid;

pub enum ClipUnit {}
pub enum LocalUnit {}
pub enum Pixel {}

pub type Scalar = f32;

pub type Angle = euclid::Angle<Scalar>;
pub type Pnt2 = euclid::Point2D<Scalar, Pixel>;
pub type Vec2 = euclid::Vector2D<Scalar, Pixel>;
pub type Rect = euclid::Rect<Scalar, Pixel>;
pub type Size = euclid::Size2D<Scalar, Pixel>;

pub type Transform<Src, Dest> = euclid::Transform2D<Scalar, Src, Dest>;
