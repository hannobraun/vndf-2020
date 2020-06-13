use std::fmt::Write as _;

use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::elements,
    },
    graphics,
};

use super::{
    TextPanel,
    TextPanelRelatedError,
};


pub struct ScaleFactor<'r>(TextPanel<'r>);

impl<'r> ScaleFactor<'r> {
    pub fn new(
        res:   &mut DrawResources,
        buf:   &'r mut String,
        frame: &Frame,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        write!(
            buf,
            "Scale factor: {}",
            frame.screen.scale_factor,
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

impl<'r> elements::Size for ScaleFactor<'r> {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}

impl<'r> elements::Draw for ScaleFactor<'r> {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw(res, frame, pos)
    }
}
