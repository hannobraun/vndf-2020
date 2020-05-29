use wgpu_glyph::{
    GlyphBrush,
    GlyphBrushBuilder,
    GlyphCruncher as _,
    Section,
};

use crate::{
    frontend::drawers::Frame,
    graphics,
};


pub struct Text {
    glyph_brush: GlyphBrush<'static, ()>,
}

impl Text {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat)
        -> Result<Self, wgpu_glyph::rusttype::Error>
    {
        let font = include_bytes!("../ui/fonts/Tuffy_Bold.ttf");
        let glyph_brush = GlyphBrushBuilder::using_font_bytes(&font[..])?
            .build(&device, format);

        Ok(
            Self {
                glyph_brush,
            }
        )
    }

    pub fn bounds(&mut self, section: &Section) -> Option<graphics::Size> {
        self.glyph_brush.glyph_bounds(section)
            .map(|size| {
                graphics::Size::new(size.width(), size.height())
            })
    }

    pub fn draw<'r>(&mut self,
        device:   &wgpu::Device,
        frame:    &mut Frame,
        sections: impl IntoIterator<Item=Section<'r>>,
    ) {
        for section in sections {
            self.glyph_brush.queue(section);
        }

        self.glyph_brush
            .draw_queued(
                &device,
                &mut frame.encoder,
                &frame.output.view,
                frame.screen.size.width as u32,
                frame.screen.size.height as u32,
            )
            // I've checked the code, and it doesn't look like this
            // actually returns any errors.
            .unwrap();
    }
}
