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
    Stack,
    TextPanelRelatedError,
};


pub struct Commands(Stack);

impl Commands {
    pub fn new(
        res:    &mut DrawResources,
        margin: graphics::Scalar,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let mut stack = Stack::new(margin);

        stack.add(CommandsList::new(res)?);

        Ok(
            Self(stack)
        )
    }
}

impl Widget for Commands {
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
