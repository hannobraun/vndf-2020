use wgpu_glyph::{
    GlyphBrush,
    GlyphBrushBuilder,
};


pub struct Ui {
    pub glyph_brush: GlyphBrush<'static, ()>,
}

impl Ui {
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
