use wgpu_glyph::{
    GlyphBrush,
    GlyphBrushBuilder,
    Scale,
    Section,
};

use crate::{
    game::Game,
    graphics::screen::Screen,
    ui,
};


pub struct Ui {
    glyph_brush: GlyphBrush<'static, ()>,
}

impl Ui {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat)
        -> Result<Self, wgpu_glyph::rusttype::Error>
    {
        let font = include_bytes!("../fonts/Tuffy_Bold.ttf");
        let glyph_brush = GlyphBrushBuilder::using_font_bytes(&font[..])?
            .build(&device, texture_format);

        Ok(
            Self {
                glyph_brush,
            }
        )
    }

    pub fn draw(&mut self,
        device:  &wgpu::Device,
        frame:   &wgpu::SwapChainOutput,
        encoder: &mut wgpu::CommandEncoder,
        game:    &Game,
        screen:  &Screen,
    ) {
        let scale  = Scale::uniform(16.0 * screen.scale_factor);

        for element in ui::elements(game, screen) {
            let text = element.text.as_str();
            let screen_position = (
                element.pos.x * screen.scale_factor,
                element.pos.y * screen.scale_factor,
            );
            let color = [1.0, 1.0, 1.0, 1.0];

            self.glyph_brush.queue(Section {
                text,
                screen_position,
                scale,
                color,
                .. Section::default()
            });
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
    }
}
