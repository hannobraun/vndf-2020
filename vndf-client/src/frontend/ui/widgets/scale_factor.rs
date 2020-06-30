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


pub struct ScaleFactor<'r>(TextPanel<'r>);

impl ScaleFactor<'_> {
    pub fn new(
        res:   &mut DrawResources,
        buf:   String,
        frame: &Frame,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let mut text = buf;
        write!(
            text,
            "Scale factor: {}",
            frame.screen.scale_factor,
        )?;

        let text_panel = TextPanel::new(
            res,
            text,
        )?;

        Ok(
            Self(text_panel)
        )
    }
}

impl Widget for ScaleFactor<'_> {
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
