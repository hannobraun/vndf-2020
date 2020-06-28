use std::fmt::Write as _;

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

use super::{
    TextPanel,
    TextPanelRelatedError,
};


pub struct Commands<'a>(TextPanel<'a>);

impl<'a> Commands<'a> {
    pub fn new(
        res: &mut DrawResources,
        buf: &'a mut String,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        write!(
            buf,
            "Commands",
        )?;

        let text_panel = TextPanel::new(
            res,
            buf,
        )?;

        Ok(
            Self(text_panel)
        )
    }
}

impl Widget for Commands<'_> {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }

    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw(res, frame, pos)
    }
}
