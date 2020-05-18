use conrod_core::{
    Color,
    Positionable as _,
    Theme,
    Ui,
    UiBuilder,
    Widget as _,
    event::Input,
    image,
    widget::{
        self,
        canvas,
    },
    widget_ids,
};
use conrod_wgpu::RenderPassCommand;
use winit::event::{
    Event,
    WindowEvent,
};

use crate::{
    game::Game,
    graphics::{
        self,
        screen::Screen,
    },
};


pub struct Conrod {
    ui:       Ui,
    ids:      Ids,
    renderer: conrod_wgpu::Renderer,
}

impl Conrod {
    pub fn new(
        device:         &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        screen_size:    graphics::Size,
    )
        -> Self
    {
        let mut ui = UiBuilder::new(screen_size.cast().to_array())
            .theme(
                Theme {
                    label_color: Color::Rgba(1.0, 1.0, 1.0, 1.0),
                    .. Theme::default()
                }
            )
            .build();

        let ids = Ids::new(ui.widget_id_generator());

        let renderer = conrod_wgpu::Renderer::new(device, 1, texture_format);

        Self {
            ui,
            ids,
            renderer,
        }
    }
}

impl super::Ui for Conrod {
    fn draw(&mut self,
        device:  &wgpu::Device,
        frame:   &wgpu::SwapChainOutput,
        encoder: &mut wgpu::CommandEncoder,
        _game:   &Game,
        screen:  &Screen,
    )
        -> Result<(), ()>
    {
        {
            let ui  = &mut self.ui.set_widgets();

            widget::Canvas::new()
                .with_style(canvas::Style {
                    color: Some(Color::Rgba(0.0, 0.0, 0.0, 0.0)),
                    .. canvas::Style::default()
                })
                .pad(20.0)
                .set(self.ids.canvas, ui);

            widget::Circle::fill(10.0)
                .top_left_of(self.ids.canvas)
                .set(self.ids.circle, ui);
        }

        let primitives = self.ui.draw();
        let image_map  = image::Map::new();

        let command = self.renderer
            .fill(
                &image_map,
                [0.0, 0.0, screen.size.width, screen.size.height],
                // This is the scale factor. I haven't quite figured out why,
                // but passing the actual scale factor from `screen` here leads
                // to weird effects. I suspect that Conrod expects to get the
                // unscaled size in the previous argument, and will scale those
                // by itself, using the scale factor given here.
                1.0,
                primitives,
            )
            .map_err(|_| ())?;
        if let Some(command) = command {
            command.load_buffer_and_encode(device, encoder);
        }

        let render = self.renderer.render(device, &image_map);

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
            }
        );

        render_pass.set_vertex_buffer(0, &render.vertex_buffer, 0, 0);

        for command in render.commands {
            match command {
                RenderPassCommand::SetPipeline { pipeline } => {
                    render_pass.set_pipeline(pipeline);
                }
                RenderPassCommand::SetBindGroup { bind_group } => {
                    render_pass.set_bind_group(0, bind_group, &[]);
                }
                RenderPassCommand::SetScissor { top_left, dimensions } => {
                    let [x, y] = top_left;
                    let [w, h] = dimensions;

                    render_pass.set_scissor_rect(x, y, w, h);
                }
                RenderPassCommand::Draw { vertex_range } => {
                    render_pass.draw(vertex_range, 0..1);
                }
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                self.ui.handle_event(
                    Input::Resize(
                        size.width  as f64,
                        size.height as f64,
                    )
                );
            }
            _ => {}
        }
    }
}


widget_ids! {
    pub struct Ids {
        canvas,
        circle,
    }
}
