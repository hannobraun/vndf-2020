use std::fmt;

use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::{
            DrawAt,
            Size,
        },
    },
    graphics,
};


pub struct Text {
    section: glyph_brush::OwnedSection,
    size:    graphics::Size,
}

impl Text {
    pub fn create(
        res:  &mut DrawResources,
        text: String,
    )
        -> Result<Self, CreateError>
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
            None       => return Err(CreateError::NoBounds),
        };

        Ok(
            Self {
                section,
                size,
            }
        )
    }
}

impl Size for Text {
    fn size(&self) -> graphics::Size {
        self.size
    }
}

impl DrawAt for Text {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.section.screen_position = (pos.x, pos.y);

        res.drawables.text.queue(&self.section);
        res.drawables.text
            .draw(
                &res.device,
                frame,
            )
            .unwrap();
    }
}


#[derive(Debug)]
pub enum CreateError {
    Fmt(fmt::Error),
    NoBounds,
}

impl From<fmt::Error> for CreateError {
    fn from(err: fmt::Error) -> Self {
        Self::Fmt(err)
    }
}
