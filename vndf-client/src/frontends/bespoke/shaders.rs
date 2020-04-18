use std::io::{
    self,
    Cursor,
};


pub fn vertex_shader(device: &wgpu::Device) -> Result {
    Shader::Vertex.load(device)
}

pub fn fragment_shader(device: &wgpu::Device) -> Result {
    Shader::Fragment.load(device)
}


enum Shader {
    Vertex,
    Fragment,
}

impl Shader {
    fn load(&self, device: &wgpu::Device) -> Result {
        let code = match self {
            Shader::Vertex => {
                &include_bytes!("shaders/spv/simple.vert.spv")[..]
            }
            Shader::Fragment => {
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
