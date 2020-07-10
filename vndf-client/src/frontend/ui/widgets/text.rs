use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::{
            DrawAt,
            Widget,
        },
    },
    graphics,
};


pub struct Text {
    section: glyph_brush::OwnedSection,
    size:    graphics::Size,
}

impl Text {
    pub fn new(
        res:  &mut DrawResources,
        text: String,
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
            }
        )
    }
}

impl Widget for Text {
    fn size(&self) -> graphics::Size {
        self.size
    }
}

impl DrawAt for Text {
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
