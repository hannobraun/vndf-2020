use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::Widget,
    },
    graphics,
};


pub struct Text<'r> {
    section: glyph_brush::OwnedSection,
    size:    graphics::Size,
    _tmp:    core::marker::PhantomData<&'r ()>,
}

impl<'r> Text<'r> {
    pub fn new(
        res:  &mut DrawResources,
        text: &'r str,
    )
        -> Result<Self, NoBoundsError>
    {
        let text = vec![
            glyph_brush::OwnedText::default()
                .with_text(text)
                .with_scale(16.0)
                .with_color([1.0, 1.0, 1.0, 1.0]),
        ];

        let section = glyph_brush::OwnedSection {
            text,
            // placeholder; position is supplied in `draw`
            screen_position: (0.0, 0.0),
            .. Default::default()
        };

        let size = match res.drawables.text.bounds(&section) {
            Some(size) => size,
            None       => return Err(NoBoundsError),
        };

        Ok(
            Self {
                section,
                size,
                _tmp: core::marker::PhantomData,
            }
        )
    }
}

impl Widget for Text<'_> {
    fn size(&self) -> graphics::Size {
        self.size
    }

    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.section.screen_position = (pos.x, pos.y);

        res.drawables.text.queue(&self.section);
        res.drawables.text.draw(
            &res.device,
            frame,
        );
    }
}


#[derive(Debug)]
pub struct NoBoundsError;