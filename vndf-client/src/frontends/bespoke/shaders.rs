use std::io::{
    self,
    Cursor,
};


pub fn vertex_shader(device: &wgpu::Device) -> Result {
    Shader::Vertex(VertexShader::Simple).load(device)
}

pub fn fragment_shader(device: &wgpu::Device) -> Result {
    Shader::Fragment(FragmentShader::Simple).load(device)
}


pub enum VertexShader {
    Simple,
}

pub enum FragmentShader {
    Simple,
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


pub type Result = std::result::Result<wgpu::ShaderModule, io::Error>;
