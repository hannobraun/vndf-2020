pub mod elements;
pub mod math;
pub mod screen;
pub mod transforms;
pub mod vertices;

pub use self::math::{Angle, Pnt2, Rect, Scalar, Size, Transform, Vec2};

pub const BACKGROUND_COLOR: wgpu::Color = wgpu::Color {
    r: 0.0,
    g: 0.0,
    b: 0.15,
    a: 1.0,
};
