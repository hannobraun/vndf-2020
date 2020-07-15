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
    game::Game,
    graphics,
};

use super::{
    Column,
    Commands,
    ShipStatus,
    TextPanelRelatedError,
};


#[derive(DrawAt)]
pub struct ShipControl(Column);

impl ShipControl {
    pub fn new(
        res:    &mut DrawResources,
        margin: graphics::Scalar,
        game:   &Game,
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
        )?;

        let mut column = Column::new(margin);

        if let Some(ship_status) = ship_status {
            column.add(ship_status);
        }
        column.add(commands);

        Ok(
            Some(
                Self(column)
            )
        )
    }
}

impl Widget for ShipControl {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}
