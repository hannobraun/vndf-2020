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
    Commands,
    ShipStatus,
    Stack,
    TextPanelRelatedError,
};


pub struct ShipControl(Stack);

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

        let mut stack = Stack::new(margin);

        if let Some(ship_status) = ship_status {
            stack.add(ship_status);
        }
        stack.add(commands);

        Ok(
            Some(
                Self(stack)
            )
        )
    }
}

impl Widget for ShipControl {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}

impl DrawAt for ShipControl {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw_at(res, frame, pos)
    }
}
