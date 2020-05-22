use wgpu_glyph::{
    GlyphBrush,
    GlyphBrushBuilder,
    Scale,
    Section,
};
use winit::event::Event;

use crate::{
    frontend::drawables::Drawables,
    game::Game,
    graphics::screen::Screen,
    ui,
};


pub struct Basic {
    glyph_brush: GlyphBrush<'static, ()>,
}

impl Basic {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat)
        -> Result<Self, wgpu_glyph::rusttype::Error>
    {
        let font = include_bytes!("fonts/Tuffy_Bold.ttf");
        let glyph_brush = GlyphBrushBuilder::using_font_bytes(&font[..])?
            .build(&device, texture_format);

        Ok(
            Self {
                glyph_brush,
            }
        )
    }
}

impl super::Ui for Basic {
    fn draw(&mut self,
        device:     &wgpu::Device,
        frame:      &wgpu::SwapChainOutput,
        encoder:    &mut wgpu::CommandEncoder,
        _drawables: &mut Drawables,
        game:       &Game,
        screen:     &Screen,
    )
        -> Result<(), ()>
    {
        let scale  = Scale::uniform(16.0 * screen.scale_factor);

        for element in ui::Elements::new(game, screen).elements() {
            let text = element.text.as_str();
            let screen_position = element.pos * screen.scale_factor;
            let color = [1.0, 1.0, 1.0, 1.0];

            let section = Section {
                text,
                screen_position: (screen_position.x, screen_position.y),
                scale,
                color,
                .. Section::default()
            };

            self.glyph_brush.queue(section);
        }

        self.glyph_brush
            .draw_queued(
                device,
                encoder,
                &frame.view,
                screen.size.width as u32,
                screen.size.height as u32,
            )
            // I've checked the code, and it doesn't look like this
            // actually returns any errors.
            .unwrap();

        Ok(())
    }

    fn handle_event(&mut self, _: &Event<()>, _: &Screen) {}
}
