use std::borrow::Cow;

use wgpu_glyph::{
    GlyphBrush,
    GlyphBrushBuilder,
    GlyphCruncher as _,
    Section,
    ab_glyph::FontRef,
};

use crate::{
    frontend::drawers::Frame,
    graphics,
};


pub struct Text {
    glyph_brush: GlyphBrush<(), FontRef<'static>>,
}

impl Text {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat)
        -> Result<Self, wgpu_glyph::ab_glyph::InvalidFont>
    {
        let font = include_bytes!("../fonts/Tuffy_Bold.ttf");
        let font = FontRef::try_from_slice(&font[..])?;
        let glyph_brush = GlyphBrushBuilder::using_font(font)
            .build(&device, format);

        Ok(
            Self {
                glyph_brush,
            }
        )
    }

    pub fn bounds<'a>(&mut self,
        section: impl Into<Cow<'a, Section<'a, wgpu_glyph::Extra>>>,
    )
        -> Option<graphics::Size>
    {
        self.glyph_brush.glyph_bounds(section)
            .map(|size| {
                graphics::Size::new(
                    size.width(),
                    size.height(),
                )
            })
    }

    pub fn queue<'a>(&mut self,
        section: impl Into<Cow<'a, Section<'a>>>,
    ) {
        self.glyph_brush.queue(section);
    }

    pub fn draw<'r>(&mut self,
        device: &wgpu::Device,
        frame:  &mut Frame,
    )
        -> Result<(), Error>
    {
        self.glyph_brush
            .draw_queued(
                &device,
                &mut wgpu::util::StagingBelt::new(256),
                &mut frame.encoder,
                &frame.output.view,
                frame.screen.logical_size().width as u32,
                frame.screen.logical_size().height as u32,
            )
            .map_err(|err| Error(err))
    }
}


#[derive(Debug)]
pub struct Error(String);
