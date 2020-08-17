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
            Pointer,
            widgets::{
                DrawAt,
                Size,
            },
        },
    },
    game::Game,
    graphics,
};

use super::{
    Column,
    Commands,
    ShipStatus,
    TextPanelRelatedError,
};


#[derive(DrawAt, Size)]
pub struct ShipControl(Column);

impl ShipControl {
    pub fn new(
        res:     &mut DrawResources,
        margin:  graphics::Scalar,
        pointer: Pointer,
        game:    &Game,
    )
        -> Result<Option<Self>, TextPanelRelatedError>
    {
        let ship_status = ShipStatus::new(
            res,
            game,
        )?;
        let commands = Commands::new(
            res,
            margin,
            pointer,
        )?;

        let mut column = Column::new(margin);

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
