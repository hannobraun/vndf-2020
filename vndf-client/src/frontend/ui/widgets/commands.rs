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

use super::{
    AddCommand,
    Column,
    CommandsList,
    TextPanelRelatedError,
};


pub struct Commands(Column);

impl Commands {
    pub fn new(
        res:    &mut DrawResources,
        margin: graphics::Scalar,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let mut column = Column::new(margin);

        column.add(CommandsList::new(res)?);
        column.add(AddCommand::new(res)?);

        Ok(
            Self(column)
        )
    }
}

impl Widget for Commands {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}

impl DrawAt for Commands {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw_at(res, frame, pos)
    }
}
