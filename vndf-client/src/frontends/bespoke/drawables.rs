use std::{
    convert::TryInto as _,
    io,
    mem::{
        size_of,
        size_of_val,
    },
};

use zerocopy::AsBytes as _;

use crate::graphics::{
    math::{
        ClipUnit,
        LocalUnit,
    },
    transforms::{
        NativeTransform,
        Transform,
    },
};

use super::{
    meshes::{
        Mesh,
        Meshes,
        Vertex,
    },
    shaders::{
        FragmentShader,
        VertexShader,
    },
};


pub struct Drawables {
    pub orbit:  Drawable,
    pub planet: Drawable,
    pub ship:   Drawable,
}

impl Drawables {
    pub fn new(device: &wgpu::Device, meshes: &Meshes)
        -> Result<Self, io::Error>
    {
        let orbit = Drawable::new(
            device,
            &meshes.square,
            VertexShader::Simple,
            FragmentShader::Simple,
        )?;
        let planet = Drawable::new(
            device,
            &meshes.square,
            VertexShader::Simple,
            FragmentShader::Planet,
        )?;
        let ship = Drawable::new(
            device,
            &meshes.ship,
            VertexShader::Simple,
            FragmentShader::Simple,
        )?;

        Ok(
            Self {
                orbit,
                planet,
                ship,
            }
        )
    }
}


pub struct Drawable {
    pub transform_buffer: wgpu::Buffer,
    pub color_buffer:     wgpu::Buffer,
    pub vertex_buffer:    wgpu::Buffer,
    pub index_buffer:     wgpu::Buffer,
    pub bind_group:       wgpu::BindGroup,
    pub render_pipeline:  wgpu::RenderPipeline,
    pub num_indices:      u32,
}

impl Drawable {
    pub fn new(
        device:          &wgpu::Device,
        mesh:            &Mesh,
        vertex_shader:   VertexShader,
        fragment_shader: FragmentShader,
    )
        -> Result<Self, io::Error>
    {
        let transform_buffer = device.create_buffer_with_data(
            Transform::<LocalUnit, ClipUnit>::identity()
                .to_native()
                .as_bytes(),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );
        let color_buffer = device.create_buffer_with_data(
            [1.0f32, 1.0, 1.0, 1.0]
                .as_bytes(),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        let vertex_buffer = device.create_buffer_with_data(
            mesh.vertices.as_bytes(),
            wgpu::BufferUsage::VERTEX,
        );
        let index_buffer = device.create_buffer_with_data(
            mesh.indices.as_bytes(),
            wgpu::BufferUsage::INDEX,
        );

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::UniformBuffer { dynamic: false},
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::UniformBuffer { dynamic: false },
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
                            buffer: &transform_buffer,
                            range: 0 .. size_of::<NativeTransform>() as u64,
                        },
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &color_buffer,
                            range: 0 .. size_of::<Color>() as u64,
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
                    module:      &vertex_shader.load(device)?,
                    entry_point: "main",
                },
                fragment_stage: Some(
                    wgpu::ProgrammableStageDescriptor {
                        module:      &fragment_shader.load(device)?,
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
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                color_states: &[
                    wgpu::ColorStateDescriptor {
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        color_blend: wgpu::BlendDescriptor {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation:  wgpu::BlendOperation::Add,
                        },
                        alpha_blend: wgpu::BlendDescriptor {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation:  wgpu::BlendOperation::Add,
                        },
                        write_mask: wgpu::ColorWrite::ALL,
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

        let num_indices = mesh.indices
            .len()
            .try_into()
            .expect("Mesh had too many indices; couldn't cast to `u32`");

        Ok(
            Self {
                transform_buffer,
                color_buffer,
                vertex_buffer,
                index_buffer,
                render_pipeline,
                bind_group,
                num_indices,
            }
        )
    }

    pub fn draw(&self,
        device:    &wgpu::Device,
        frame:     &wgpu::SwapChainOutput,
        encoder:   &mut wgpu::CommandEncoder,
        transform: NativeTransform,
        color:     Color,
    ) {
        // Copy transform to buffer
        let buffer = device.create_buffer_with_data(
            transform.as_bytes(),
            wgpu::BufferUsage::COPY_SRC,
        );
        encoder.copy_buffer_to_buffer(
            &buffer, 0,
            &self.transform_buffer, 0,
            size_of_val(&transform) as u64,
        );

        // Copy color to buffer
        let buffer = device.create_buffer_with_data(
            color.as_bytes(),
            wgpu::BufferUsage::COPY_SRC,
        );
        encoder.copy_buffer_to_buffer(
            &buffer, 0,
            &self.color_buffer, 0,
            size_of_val(&color) as u64,
        );

        let mut render_pass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment:     &frame.view,
                        resolve_target: None,
                        load_op:        wgpu::LoadOp::Load,
                        store_op:       wgpu::StoreOp::Store,
                        clear_color:    wgpu::Color::TRANSPARENT,
                    },
                ],
                depth_stencil_attachment: None,
            },
        );
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
        render_pass.set_index_buffer(&self.index_buffer, 0, 0);
        render_pass.draw_indexed(
            0 .. self.num_indices,
            0,
            0 .. 1,
        );
    }
}


pub type Color = [f32; 4];
