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
}

impl<'r> Text<'r> {
    pub fn new(
        text: &'r str,
        pos:  graphics::Pnt2,
    )
        -> Self
    {
        let text = vec![
            wgpu_glyph::Text::default()
                .with_text(text)
                .with_scale(16.0)
                .with_color([1.0, 1.0, 1.0, 1.0]),
        ];

        let section = wgpu_glyph::Section {
            text,
            screen_position: (pos.x, pos.y),
            .. wgpu_glyph::Section::default()
        };

        Self {
            section,
        }
    }
}

impl<'r> Element for Text<'r> {
    fn size(&self, res: &mut DrawResources)
        -> Option<graphics::Size>
    {
        res.drawables.text.bounds(&self.section)
    }

    fn draw(self,
        res:   &mut DrawResources,
        frame: &mut Frame,
    ) {
        res.drawables.text.draw(
            &res.device,
            frame,
            Some(self.section),
        );
    }
}
