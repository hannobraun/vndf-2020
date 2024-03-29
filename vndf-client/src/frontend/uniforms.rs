use zerocopy::AsBytes;

use crate::graphics::transforms::Transform;

#[derive(AsBytes)]
#[repr(packed)]
pub struct Color(pub [f32; 4]);

impl Default for Color {
    fn default() -> Self {
        Color([1.0; 4])
    }
}

impl From<[f32; 3]> for Color {
    fn from([r, g, b]: [f32; 3]) -> Self {
        Color([r, g, b, 1.0])
    }
}

impl From<[f32; 4]> for Color {
    fn from(color: [f32; 4]) -> Self {
        Color(color)
    }
}

pub type Float = f32;

#[derive(AsBytes)]
#[repr(packed)]
pub struct Mat4(pub [[f32; 4]; 4]);

impl Default for Mat4 {
    fn default() -> Self {
        Transform::<(), ()>::identity().into()
    }
}

impl<A, B> From<Transform<A, B>> for Mat4 {
    fn from(transform: Transform<A, B>) -> Self {
        Mat4(transform.to_native())
    }
}

#[derive(AsBytes)]
#[repr(packed)]
pub struct Vec2(pub [f32; 2]);

impl Default for Vec2 {
    fn default() -> Self {
        Vec2([1.0, 1.0])
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(vec2: [f32; 2]) -> Self {
        Vec2(vec2)
    }
}
