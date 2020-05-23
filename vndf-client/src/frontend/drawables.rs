use std::{
    convert::TryInto as _,
    io,
    marker::PhantomData,
    mem::{
        size_of,
        size_of_val,
    },
};

use zerocopy::AsBytes as _;

use super::{
    drawers::FrameResources,
    meshes::{
        Mesh,
        Meshes,
        Vertex,
    },
    shaders::{
        self,
        Shader,
        frag,
        vert,
    },
};


pub struct Drawables {
    pub explosion: Drawable<vert::Simple, frag::Explosion>,
    pub orbit:     Drawable<vert::Simple, frag::Orbit>,
    pub panel:     Drawable<vert::Simple, frag::Simple>,
    pub planet:    Drawable<vert::Simple, frag::Planet>,
    pub ship:      Drawable<vert::Simple, frag::Simple>,
}

impl Drawables {
    pub fn new(device: &wgpu::Device, meshes: &Meshes)
        -> Result<Self, io::Error>
    {
        let explosion = Drawable::new(
            device,
            &meshes.square,
        )?;
        let orbit = Drawable::new(
            device,
            &meshes.square,
        )?;
        let panel = Drawable::new(
            device,
            &meshes.square,
        )?;
        let planet = Drawable::new(
            device,
            &meshes.square,
        )?;
        let ship = Drawable::new(
            device,
            &meshes.ship,
        )?;

        Ok(
            Self {
                explosion,
                orbit,
                panel,
                planet,
                ship,
            }
        )
    }
}


pub struct Drawable<Vert, Frag> {
    vert_uniforms:   wgpu::Buffer,
    frag_uniforms:   wgpu::Buffer,
    vertex_buffer:   wgpu::Buffer,
    index_buffer:    wgpu::Buffer,
    bind_group:      wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    num_indices:     u32,

    vert: PhantomData<Vert>,
    frag: PhantomData<Frag>,
}

impl<Vert, Frag> Drawable<Vert, Frag>
    where
        Vert: Shader<Kind=shaders::Vert>,
        Frag: Shader<Kind=shaders::Frag>,
{
    pub fn new(
        device: &wgpu::Device,
        mesh:   &Mesh,
    )
        -> Result<Self, io::Error>
    {
        let vert_uniforms = device.create_buffer_with_data(
            Vert::Uniforms::default()
                .as_bytes(),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );
        let frag_uniforms = device.create_buffer_with_data(
            Frag::Uniforms::default()
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
                        ty: wgpu::BindingType::UniformBuffer { dynamic: false },
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
                            buffer: &vert_uniforms,
                            range: 0 .. size_of::<Vert::Uniforms>() as u64,
                        },
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &frag_uniforms,
                            range: 0 .. size_of::<Frag::Uniforms>() as u64,
                        },
                    },
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
                    module:      &Vert::load(device)?,
                    entry_point: "main",
                },
                fragment_stage: Some(
                    wgpu::ProgrammableStageDescriptor {
                        module:      &Frag::load(device)?,
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
                vert_uniforms,
                frag_uniforms,
                vertex_buffer,
                index_buffer,
                render_pipeline,
                bind_group,
                num_indices,

                vert: PhantomData,
                frag: PhantomData,
            }
        )
    }

    pub fn draw(&self,
        device:    &wgpu::Device,
        res:       &mut FrameResources,
        vert_args: Vert::Uniforms,
        frag_args: Frag::Uniforms,
    ) {
        let buffer = device.create_buffer_with_data(
            vert_args.as_bytes(),
            wgpu::BufferUsage::COPY_SRC,
        );
        res.encoder.copy_buffer_to_buffer(
            &buffer, 0,
            &self.vert_uniforms, 0,
            size_of_val(&vert_args) as u64,
        );

        let buffer = device.create_buffer_with_data(
            frag_args.as_bytes(),
            wgpu::BufferUsage::COPY_SRC,
        );
        res.encoder.copy_buffer_to_buffer(
            &buffer, 0,
            &self.frag_uniforms, 0,
            size_of_val(&frag_args) as u64,
        );

        let mut render_pass = res.encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment:     &res.output.view,
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
