pub mod math;
pub mod model;
pub mod transforms;
pub mod vertices;


pub use self::{
    math::{
        Pnt2,
        Size,
        Vec2,
    },
    model::Model,
};


pub const BACKGROUND_COLOR: wgpu::Color =
    wgpu::Color { r: 0.0, g: 0.0, b: 0.15, a: 1.0 };
