use std::io::{
    self,
    Cursor,
};


pub enum VertexShader {
    Simple,
}

impl VertexShader {
    pub fn load(self, device: &wgpu::Device) -> Result {
        Shader::load(&self.into(), device)
    }
}


pub enum FragmentShader {
    Orbit,
    Planet,
    Simple,
}

impl FragmentShader {
    pub fn load(self, device: &wgpu::Device) -> Result {
        Shader::load(&self.into(), device)
    }
}


enum Shader {
    Vertex(VertexShader),
    Fragment(FragmentShader),
}

impl Shader {
    fn load(&self, device: &wgpu::Device) -> Result {
        let code = match self {
            Shader::Vertex(VertexShader::Simple) => {
                &include_bytes!("shaders/spv/simple.vert.spv")[..]
            }
            Shader::Fragment(FragmentShader::Orbit) => {
                &include_bytes!("shaders/spv/orbit.frag.spv")[..]
            }
            Shader::Fragment(FragmentShader::Planet) => {
                &include_bytes!("shaders/spv/planet.frag.spv")[..]
            }
            Shader::Fragment(FragmentShader::Simple) => {
                &include_bytes!("shaders/spv/simple.frag.spv")[..]
            }
        };

        load(code, device)
    }
}

impl From<VertexShader> for Shader {
    fn from(vert: VertexShader) -> Self {
        Self::Vertex(vert)
    }
}

impl From<FragmentShader> for Shader {
    fn from(frag: FragmentShader) -> Self {
        Self::Fragment(frag)
    }
}


fn load(code: &[u8], device: &wgpu::Device) -> Result {
    let module = device.create_shader_module(
        &wgpu::read_spirv(Cursor::new(code))?,
    );

    Ok(module)
}


pub type Result = std::result::Result<wgpu::ShaderModule, io::Error>;
