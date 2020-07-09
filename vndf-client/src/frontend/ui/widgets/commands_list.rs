use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::{
            Draw,
            Widget,
        },
    },
    graphics,
};

use super::{
    TextPanel,
    TextPanelRelatedError,
};


pub struct CommandsList(TextPanel);

impl CommandsList {
    pub fn new(
        res: &mut DrawResources,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let text_panel = TextPanel::new(
            res,
            format!(
                "Commands",
            ),
        )?;

        Ok(
            Self(text_panel)
        )
    }
}

impl Widget for CommandsList {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}

impl Draw for CommandsList {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw(res, frame, pos)
    }
}
