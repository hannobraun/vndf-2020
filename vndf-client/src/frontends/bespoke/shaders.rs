use std::io::{
    self,
    Cursor,
};


pub fn vertex_shader(device: &wgpu::Device) -> Result {
    VertexShader::Simple.load(device)
}

pub fn fragment_shader(device: &wgpu::Device) -> Result {
    FragmentShader::Simple.load(device)
}


pub enum VertexShader {
    Simple,
}

impl VertexShader {
    pub fn load(self, device: &wgpu::Device) -> Result {
        Shader::load(&self.into(), device)
    }
}


pub enum FragmentShader {
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
            Shader::Fragment(FragmentShader::Simple) => {
                &include_bytes!("shaders/spv/simple.frag.spv")[..]
            }
        };

        let module = device.create_shader_module(
            &wgpu::read_spirv(Cursor::new(code))?,
        );

        Ok(module)
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


pub type Result = std::result::Result<wgpu::ShaderModule, io::Error>;
