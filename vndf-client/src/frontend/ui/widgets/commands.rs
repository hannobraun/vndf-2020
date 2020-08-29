use vndf_macros::{
    DrawAt,
    Size,
};

use crate::{
    frontend::{
        drawers::DrawResources,
        ui::input::Input,
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
    pub fn create(
        res:    &mut DrawResources,
        margin: graphics::Scalar,
        input:  &Input,
    )
        -> Result<Self, text::CreateError>
    {
        let mut column = Column::create(margin);

        column.add(CommandsList::create(res)?);
        column.add(AddCommand::create(res, input)?);

        Ok(
            Self(column)
        )
    }
}
