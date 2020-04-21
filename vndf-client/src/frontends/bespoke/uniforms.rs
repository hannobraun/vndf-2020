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
            color:       [1.0; 4],
            u_per_pixel: [1.0, 1.0],
        }
    }
}


pub type Color = [f32; 4];
