pub mod math;
pub mod vertices;


pub use math::{
    Pnt2,
    Vec2,
};


pub const BACKGROUND_COLOR: wgpu::Color =
    wgpu::Color { r: 0.0, g: 0.0, b: 0.15, a: 1.0 };
