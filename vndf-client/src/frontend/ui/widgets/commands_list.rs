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
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let mut text = String::new();
        write!(
            text,
            "Commands",
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
