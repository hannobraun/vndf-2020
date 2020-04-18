use std::io::{
    self,
    Cursor,
};


pub fn vertex_shader(device: &wgpu::Device) -> Result {
    let code = include_bytes!("shaders/shader.vert.spv");
    let module = device.create_shader_module(
        &wgpu::read_spirv(Cursor::new(&code[..]))?,
    );
    Ok(module)
}


pub type Result = std::result::Result<wgpu::ShaderModule, io::Error>;
