use vndf_macros::{
    DrawAt,
    ProcessInputAt,
    Size,
};

use crate::{
    frontend::drawers::DrawResources,
    game::Game,
    graphics,
};

use super::{
    Column,
    Commands,
    ShipStatus,
    text,
};


#[derive(DrawAt, ProcessInputAt, Size)]
pub struct ShipControl(Column);

impl ShipControl {
    pub fn create(
        res:    &mut DrawResources,
        margin: graphics::Scalar,
        game:   &Game,
    )
        -> Result<Option<Self>, text::CreateError>
    {
        let ship_status = ShipStatus::create(
            res,
            game,
        )?;
        let commands = Commands::create(
            res,
            margin,
        )?;

        let mut column = Column::create(margin);

        if let Some(ship_status) = ship_status {
            column.add(ship_status);
            column.add(commands);
        }

        Ok(
            Some(
                Self(column)
            )
        )
    }
}
