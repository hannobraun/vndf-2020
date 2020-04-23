use zerocopy::AsBytes;

use crate::graphics::{
    math::{
        ClipUnit,
        LocalUnit,
    },
    transforms::{
        NativeTransform,
        Transform,
    },
};


#[derive(AsBytes)]
#[repr(packed)]
pub struct Uniforms {
    pub transform:   NativeTransform,
    pub color:       Color,
    pub u_per_pixel: [f32; 2],
}

impl Default for Uniforms {
    fn default() -> Self {
        let transform = Transform::<LocalUnit, ClipUnit>::identity()
            .to_native();

        Self {
            transform,
            color:       Color::default(),
            u_per_pixel: [1.0, 1.0],
        }
    }
}


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
