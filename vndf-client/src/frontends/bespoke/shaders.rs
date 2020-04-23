use std::io::{
    self,
    Cursor,
};


pub trait Shader {
    fn code(&self) -> &'static [u8];

    fn load(&self, device: &wgpu::Device) -> Result {
        let code = self.code();

        let module = device.create_shader_module(
            &wgpu::read_spirv(Cursor::new(code))?,
        );

        Ok(module)
    }
}


macro_rules! shader {
    ($name:ident, $path:expr) => {
        pub struct $name;

        impl crate::frontends::bespoke::shaders::Shader for $name {
            fn code(&self) -> &'static [u8] {
                &include_bytes!($path)[..]
            }
        }
    };
}


pub mod vert {
    shader!(Simple, "shaders/spv/simple.vert.spv");
}

pub mod frag {
    shader!(Orbit,  "shaders/spv/orbit.frag.spv" );
    shader!(Planet, "shaders/spv/planet.frag.spv");
    shader!(Simple, "shaders/spv/simple.frag.spv");
}


pub type Result = std::result::Result<wgpu::ShaderModule, io::Error>;
