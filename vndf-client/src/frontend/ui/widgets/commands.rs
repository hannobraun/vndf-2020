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
            input::Input,
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
    text,
};


#[derive(DrawAt, Size)]
pub struct Commands(Column);

impl Commands {
    pub fn new(
        res:    &mut DrawResources,
        margin: graphics::Scalar,
        input:  &Input,
    )
        -> Result<Self, text::CreateError>
    {
        let mut column = Column::new(margin);

        column.add(CommandsList::new(res)?);
        column.add(AddCommand::new(res, input)?);

        Ok(
            Self(column)
        )
    }
}
