use vndf_macros::DrawAt;

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


#[derive(DrawAt)]
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
