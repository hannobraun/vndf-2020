use vndf_macros::{
    DrawAt,
    Size,
};

use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::{
            input::Pointer,
            widgets::{
                DrawAt,
                Size,
            },
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


#[derive(DrawAt, Size)]
pub struct Commands(Column);

impl Commands {
    pub fn new(
        res:     &mut DrawResources,
        margin:  graphics::Scalar,
        pointer: Pointer,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let mut column = Column::new(margin);

        column.add(CommandsList::new(res)?);
        column.add(AddCommand::new(res, pointer)?);

        Ok(
            Self(column)
        )
    }
}
