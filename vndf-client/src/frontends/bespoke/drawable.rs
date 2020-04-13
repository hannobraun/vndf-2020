pub struct Drawable {
    pub uniform_buffer:  wgpu::Buffer,
    pub vertex_buffer:   wgpu::Buffer,
    pub index_buffer:    wgpu::Buffer,
    pub bind_group:      wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
}
