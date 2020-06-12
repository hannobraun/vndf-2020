use crate::{
    frontend::drawers::{
        DrawResources,
        Frame,
    },
    graphics,
};

use super::Element;


pub struct Text<'r> {
    section: wgpu_glyph::Section<'r>,
    size:    graphics::Size,
}

impl<'r> Text<'r> {
    pub fn new(
        res:  &mut DrawResources,
        text: &'r str,
    )
        -> Result<Self, NoBoundsError>
    {
        let text = vec![
            wgpu_glyph::Text::default()
                .with_text(text)
                .with_scale(16.0)
                .with_color([1.0, 1.0, 1.0, 1.0]),
        ];

        let section = wgpu_glyph::Section {
            text,
            // placeholder; posisition is supplied in `draw`
            screen_position: (0.0, 0.0),
            .. wgpu_glyph::Section::default()
        };

        let size = match res.drawables.text.bounds(&section) {
            Some(size) => size,
            None       => return Err(NoBoundsError),
        };

        Ok(
            Self {
                section,
                size,
            }
        )
    }
}

impl<'r> Element for Text<'r> {
    fn size(&self) -> graphics::Size {
        self.size
    }

    fn draw(mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.section.screen_position = (pos.x, pos.y);

        res.drawables.text.draw(
            &res.device,
            frame,
            Some(self.section),
        );
    }
}


#[derive(Debug)]
pub struct NoBoundsError;