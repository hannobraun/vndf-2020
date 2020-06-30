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
    CommandsList,
    TextPanelRelatedError,
};


pub struct Commands<'a>(CommandsList<'a>);

impl Commands<'_> {
    pub fn new(
        res: &mut DrawResources,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let list = CommandsList::new(res)?;

        Ok(
            Self(list)
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
