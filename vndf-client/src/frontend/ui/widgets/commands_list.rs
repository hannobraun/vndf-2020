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


pub struct CommandsList<'a>(TextPanel<'a>);

impl CommandsList<'_> {
    pub fn new(
        res: &mut DrawResources,
        buf: String,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let mut buf = buf;
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

impl Widget for CommandsList<'_> {
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
