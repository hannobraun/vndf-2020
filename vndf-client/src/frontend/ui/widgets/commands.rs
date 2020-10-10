use vndf_macros::{
    DrawAt,
    ProcessInputAt,
    Size,
};

use crate::{
    frontend::drawers::DrawResources,
    graphics,
};

use super::{
    AddCommand,
    Column,
    CommandsList,
    text,
};


#[derive(DrawAt, ProcessInputAt, Size)]
pub struct Commands(Column);

impl Commands {
    pub fn create(
        res:    &mut DrawResources,
        margin: graphics::Scalar,
    )
        -> Result<Self, text::CreateError>
    {
        let mut column = Column::create(margin);

        column.add(CommandsList::create(res, margin)?);
        column.add(AddCommand::create(res)?);

        Ok(
            Self(column)
        )
    }
}
