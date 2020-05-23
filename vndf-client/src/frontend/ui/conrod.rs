use conrod_core::{
    Color,
    Positionable as _,
    Theme,
    Ui,
    UiBuilder,
    Widget as _,
    event::Input,
    image,
    text::Font,
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
    frontend::{
        drawables::Drawables,
        drawers::FrameResources,
    },
    game::Game,
    graphics::screen::Screen,
    ui,
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
        screen:         &Screen,
    )
        -> Result<Self, rusttype::Error>
    {
        let mut ui = UiBuilder::new(screen.logical_size().cast().to_array())
            .theme(
                Theme {
                    label_color:      Color::Rgba(1.0, 1.0, 1.0, 1.0),
                    font_size_medium: 12,
                    .. Theme::default()
                }
            )
            .build();

        let font = include_bytes!("fonts/Tuffy_Bold.ttf");
        let font = Font::from_bytes(&font[..])?;
        ui.fonts.insert(font);

        let ids = Ids::new(ui.widget_id_generator());

        let renderer = conrod_wgpu::Renderer::new(device, 1, texture_format);

        Ok(
            Self {
                ui,
                ids,
                renderer,
            }
        )
    }
}

impl super::Ui for Conrod {
    fn draw(&mut self,
        device:     &wgpu::Device,
        res:        &mut FrameResources,
        _drawables: &mut Drawables,
        game:       &Game,
    )
        -> Result<(), ()>
    {
        let elements = ui::Elements::new(game, &res.screen);

        {
            const PADDING: f64 = 20.0;

            let ui = &mut self.ui.set_widgets();

            widget::Canvas::new()
                .with_style(canvas::Style {
                    color: Some(Color::Rgba(0.0, 0.0, 0.0, 0.0)),
                    .. canvas::Style::default()
                })
                .pad(PADDING)
                .set(self.ids.canvas, ui);

            widget::Text::new(elements.instructions.text.as_str())
                .top_left_of(self.ids.canvas)
                .set(self.ids.instructions, ui);

            widget::Text::new(elements.zoom.text.as_str())
                .down(PADDING)
                .set(self.ids.zoom, ui);

            if let Some(frame_time) = elements.frame_time {
                widget::Text::new(frame_time.text.as_str())
                    .down(PADDING)
                    .set(self.ids.frame_time, ui);
            }

            if let Some(diagnostics) = elements.diagnostics {
                widget::Text::new(diagnostics.text.as_str())
                    .down(PADDING)
                    .set(self.ids.diagnostics, ui);
            }

            if let Some(input_events) = elements.input_events {
                widget::Text::new(input_events.text.as_str())
                    .down(PADDING)
                    .set(self.ids.input_events, ui);
            }

            if let Some(own_ship_status) = elements.own_ship_status {
                widget::Text::new(own_ship_status.text.as_str())
                    .top_right_of(self.ids.canvas)
                    .set(self.ids.own_ship_status, ui);
            }
        }

        let primitives = self.ui.draw();
        let image_map  = image::Map::new();

        let command = self.renderer
            .fill(
                &image_map,
                [0.0, 0.0, res.screen.size.width, res.screen.size.height],
                res.screen.scale_factor as f64,
                primitives,
            )
            .map_err(|_| ())?;
        if let Some(command) = command {
            command.load_buffer_and_encode(device, &mut res.encoder);
        }

        let render = self.renderer.render(device, &image_map);

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

    fn handle_event(&mut self, event: &Event<()>, screen: &Screen) {
        match event {
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                self.ui.handle_event(
                    Input::Resize(
                        size.width  as f64 / screen.scale_factor as f64,
                        size.height as f64 / screen.scale_factor as f64,
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
        instructions,
        zoom,
        frame_time,
        diagnostics,
        input_events,
        own_ship_status,
    }
}
