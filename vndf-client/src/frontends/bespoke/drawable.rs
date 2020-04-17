use std::{
    io::{
        self,
        Cursor,
    },
    mem::size_of,
};

use zerocopy::AsBytes as _;

use crate::graphics::{
    elements::Transform,
    math::{
        ClipUnit,
        LocalUnit,
    },
    transforms,
};

use super::meshes::{
    Meshes,
    Vertex,
};


pub struct Drawable {
    pub uniform_buffer:  wgpu::Buffer,
    pub vertex_buffer:   wgpu::Buffer,
    pub index_buffer:    wgpu::Buffer,
    pub bind_group:      wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl Drawable {
    pub fn new(device: &wgpu::Device, meshes: &Meshes)
        -> Result<Self, io::Error>
    {
        let uniform_buffer = device.create_buffer_with_data(
            transforms::Transform::<LocalUnit, ClipUnit>::identity()
                .to_3d()
                .to_row_arrays()
                .as_bytes(),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        let vertex_buffer = device.create_buffer_with_data(
            meshes.ship.vertices.as_bytes(),
            wgpu::BufferUsage::VERTEX,
        );
        let index_buffer = device.create_buffer_with_data(
            meshes.ship.indices.as_bytes(),
            wgpu::BufferUsage::INDEX,
        );

        let vertex_shader = include_bytes!("shaders/shader.vert.spv");
        let vertex_module_module = device.create_shader_module(
            &wgpu::read_spirv(Cursor::new(&vertex_shader[..]))?,
        );

        let fragment_shader = include_bytes!("shaders/shader.frag.spv");
        let fragment_module_module = device.create_shader_module(
            &wgpu::read_spirv(Cursor::new(&fragment_shader[..]))?,
        );

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::UniformBuffer { dynamic: false},
                    },
                ],
                label: None,
            },
        );
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout:   &bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &uniform_buffer,
                            range: 0 .. size_of::<Transform>() as u64,
                        },
                    }
                ],
                label: None,
            },
        );
        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
            },
        );

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                layout: &pipeline_layout,
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module:      &vertex_module_module,
                    entry_point: "main",
                },
                fragment_stage: Some(
                    wgpu::ProgrammableStageDescriptor {
                        module:      &fragment_module_module,
                        entry_point: "main",
                    }
                ),
                rasterization_state: Some(
                    wgpu::RasterizationStateDescriptor {
                        front_face:             wgpu::FrontFace::Ccw,
                        cull_mode:              wgpu::CullMode::None,
                        depth_bias:             0,
                        depth_bias_slope_scale: 0.0,
                        depth_bias_clamp:       0.0,
                    }
                ),
                primitive_topology: wgpu::PrimitiveTopology::TriangleStrip,
                color_states: &[
                    wgpu::ColorStateDescriptor {
                        format:      wgpu::TextureFormat::Bgra8UnormSrgb,
                        color_blend: wgpu::BlendDescriptor::REPLACE,
                        alpha_blend: wgpu::BlendDescriptor::REPLACE,
                        write_mask:  wgpu::ColorWrite::ALL,
                    },
                ],
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format:   wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[
                        wgpu::VertexBufferDescriptor {
                            stride: size_of::<Vertex>() as u64,
                            step_mode: wgpu::InputStepMode::Vertex,
                            attributes: &[
                                wgpu::VertexAttributeDescriptor {
                                    format: wgpu::VertexFormat::Float2,
                                    offset: 0,
                                    shader_location: 0,
                                },
                            ],
                        },
                    ],
                },
                sample_count:              1,
                sample_mask:               !0,
                alpha_to_coverage_enabled: false,
            },
        );

        Ok(
            Self {
                uniform_buffer,
                vertex_buffer,
                index_buffer,
                render_pipeline,
                bind_group,
            }
        )
    }
}