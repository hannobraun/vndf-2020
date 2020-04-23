use std::io::{
    self,
    Cursor,
};

use zerocopy::AsBytes;


pub trait Shader {
    type Kind;
    type Uniforms: AsBytes;

    fn code() -> &'static [u8];

    fn load(device: &wgpu::Device)
        -> Result<wgpu::ShaderModule, io::Error>
    {
        let code = Self::code();

        let module = device.create_shader_module(
            &wgpu::read_spirv(Cursor::new(code))?,
        );

        Ok(module)
    }
}


pub struct Vert;
pub struct Frag;


macro_rules! shader {
    (
        $kind:ty,
        $name_s:ident,
        $path:expr,
    ) => {
        pub struct $name_s;

        impl crate::frontends::bespoke::shaders::Shader for $name_s {
            type Kind     = $kind;
            type Uniforms = crate::frontends::bespoke::uniforms::Uniforms;

            fn code() -> &'static [u8] {
                &include_bytes!($path)[..]
            }
        }
    };
}

macro_rules! vertex_shader {
    (
        $name_s:ident,
        $path:expr,
    ) => {
        shader!(
            crate::frontends::bespoke::shaders::Vert,
            $name_s,
            $path,
        );
    };
}

macro_rules! fragment_shader {
    (
        $name_s:ident,
        $path:expr,
    ) => {
        shader!(
            crate::frontends::bespoke::shaders::Frag,
            $name_s,
            $path,
        );
    };
}


pub mod vert {
    vertex_shader!(
        Simple,
        "shaders/spv/simple.vert.spv",
    );
}

pub mod frag {
    fragment_shader!(
        Orbit,
        "shaders/spv/orbit.frag.spv",
    );
    fragment_shader!(
        Planet,
        "shaders/spv/planet.frag.spv",
    );
    fragment_shader!(
        Simple,
        "shaders/spv/simple.frag.spv",
    );
}
