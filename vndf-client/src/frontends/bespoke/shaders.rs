use std::io::{
    self,
    Cursor,
};


pub enum VertexShader {
    Simple,
}

impl VertexShader {
    pub fn code(&self) -> &'static [u8] {
        match self {
            Self::Simple => {
                &include_bytes!("shaders/spv/simple.vert.spv")[..]
            }
        }
    }

    pub fn load(self, device: &wgpu::Device) -> Result {
        load(self.code(), device)
    }
}


pub enum FragmentShader {
    Orbit,
    Planet,
    Simple,
}

impl FragmentShader {
    pub fn code(&self) -> &'static [u8] {
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

    pub fn load(self, device: &wgpu::Device) -> Result {
        load(self.code(), device)
    }
}


fn load(code: &[u8], device: &wgpu::Device) -> Result {
    let module = device.create_shader_module(
        &wgpu::read_spirv(Cursor::new(code))?,
    );

    Ok(module)
}


pub type Result = std::result::Result<wgpu::ShaderModule, io::Error>;
