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
    pub transform: NativeTransform,
    pub color:     Color,
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            transform: Transform::<LocalUnit, ClipUnit>::identity().to_native(),
            color:     [0.0; 4],
        }
    }
}


pub type Color = [f32; 4];
