use std::{
    convert::TryInto as _,
    io,
    marker::PhantomData,
    mem::{size_of, size_of_val},
};

use wgpu::util::DeviceExt as _;
use zerocopy::AsBytes as _;

use crate::frontend::{
    drawers::Frame,
    meshes::{Mesh, Vertex},
    shaders::{self, Shader},
};

pub struct Standard<Vert, Frag> {
    vert_uniforms: wgpu::Buffer,
    frag_uniforms: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    num_indices: u32,

    vert: PhantomData<Vert>,
    frag: PhantomData<Frag>,
}

impl<Vert, Frag> Standard<Vert, Frag>
where
    Vert: Shader<Kind = shaders::Vert>,
    Frag: Shader<Kind = shaders::Frag>,
{
    pub fn new(device: &wgpu::Device, mesh: &Mesh) -> Result<Self, io::Error> {
        let vert_uniforms = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: Vert::Uniforms::default().as_bytes(),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });
        let frag_uniforms = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: Frag::Uniforms::default().as_bytes(),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: mesh.vertices.as_bytes(),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: mesh.indices.as_bytes(),
            usage: wgpu::BufferUsage::INDEX,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: wgpu::BufferSize::new(size_of::<Vert::Uniforms>() as u64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: wgpu::BufferSize::new(size_of::<Frag::Uniforms>() as u64),
                    },
                    count: None,
                },
            ],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(vert_uniforms.slice(..)),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(frag_uniforms.slice(..)),
                },
            ],
            label: None,
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &Vert::load(device)?,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &Frag::load(device)?,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor::default()),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                color_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[wgpu::VertexBufferDescriptor {
                    stride: size_of::<Vertex>() as u64,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &[wgpu::VertexAttributeDescriptor {
                        format: wgpu::VertexFormat::Float2,
                        offset: 0,
                        shader_location: 0,
                    }],
                }],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        let num_indices = mesh
            .indices
            .len()
            .try_into()
            .expect("Mesh had too many indices; couldn't cast to `u32`");

        Ok(Self {
            vert_uniforms,
            frag_uniforms,
            vertex_buffer,
            index_buffer,
            render_pipeline,
            bind_group,
            num_indices,

            vert: PhantomData,
            frag: PhantomData,
        })
    }

    pub fn draw(
        &self,
        device: &wgpu::Device,
        frame: &mut Frame,
        vert_args: Vert::Uniforms,
        frag_args: Frag::Uniforms,
    ) {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: vert_args.as_bytes(),
            usage: wgpu::BufferUsage::COPY_SRC,
        });
        frame.encoder.copy_buffer_to_buffer(
            &buffer,
            0,
            &self.vert_uniforms,
            0,
            size_of_val(&vert_args) as u64,
        );

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: frag_args.as_bytes(),
            usage: wgpu::BufferUsage::COPY_SRC,
        });
        frame.encoder.copy_buffer_to_buffer(
            &buffer,
            0,
            &self.frag_uniforms,
            0,
            size_of_val(&frag_args) as u64,
        );

        let mut render_pass = frame
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..));
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
