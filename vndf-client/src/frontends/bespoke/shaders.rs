use std::io::{
    self,
    Cursor,
};


pub trait Shader {
    type Kind;

    fn code(&self) -> &'static [u8];

    fn load(&self, device: &wgpu::Device)
        -> Result<wgpu::ShaderModule, io::Error>
    {
        let code = self.code();

        let module = device.create_shader_module(
            &wgpu::read_spirv(Cursor::new(code))?,
        );

        Ok(module)
    }
}


pub struct Vert;
pub struct Frag;


macro_rules! shader {
    ($kind:ty, $name:ident, $path:expr) => {
        pub struct $name;

        impl crate::frontends::bespoke::shaders::Shader for $name {
            type Kind = $kind;

            fn code(&self) -> &'static [u8] {
                &include_bytes!($path)[..]
            }
        }
    };
}

macro_rules! vertex_shader {
    ($name:ident, $path:expr) => {
        shader!(crate::frontends::bespoke::shaders::Vert, $name, $path);
    };
}

macro_rules! fragment_shader {
    ($name:ident, $path:expr) => {
        shader!(crate::frontends::bespoke::shaders::Frag, $name, $path);
    };
}


pub mod vert {
    vertex_shader!(Simple, "shaders/spv/simple.vert.spv");
}

pub mod frag {
    fragment_shader!(Orbit,  "shaders/spv/orbit.frag.spv" );
    fragment_shader!(Planet, "shaders/spv/planet.frag.spv");
    fragment_shader!(Simple, "shaders/spv/simple.frag.spv");
}
