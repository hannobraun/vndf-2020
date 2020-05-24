use wgpu_glyph::{
    GlyphBrush,
    GlyphBrushBuilder,
};


pub struct Text {
    pub glyph_brush: GlyphBrush<'static, ()>,
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
}
