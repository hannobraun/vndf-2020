use std::io::{
    self,
    Cursor,
};


pub trait Shader {
    fn code(&self) -> &'static [u8];

    fn load(&self, device: &wgpu::Device) -> Result {
        load(self.code(), device)
    }
}


pub enum VertexShader {
    Simple,
}

impl Shader for VertexShader {
    fn code(&self) -> &'static [u8] {
        match self {
            Self::Simple => {
                &include_bytes!("shaders/spv/simple.vert.spv")[..]
            }
        }
    }
}


pub enum FragmentShader {
    Orbit,
    Planet,
    Simple,
}

impl Shader for FragmentShader {
    fn code(&self) -> &'static [u8] {
        match self {
            FragmentShader::Orbit => {
                &include_bytes!("shaders/spv/orbit.frag.spv")[..]
            }
            FragmentShader::Planet => {
                &include_bytes!("shaders/spv/planet.frag.spv")[..]
            }
            FragmentShader::Simple => {
                &include_bytes!("shaders/spv/simple.frag.spv")[..]
            }
        }
    }
}


fn load(code: &[u8], device: &wgpu::Device) -> Result {
    let module = device.create_shader_module(
        &wgpu::read_spirv(Cursor::new(code))?,
    );

    Ok(module)
}


pub type Result = std::result::Result<wgpu::ShaderModule, io::Error>;
