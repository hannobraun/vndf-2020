use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::Widget,
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


pub struct ShipControl<'a, 'b>(Stack<'a, 'b>);

impl<'a, 'b> ShipControl<'a, 'b> {
    pub fn new(
        res:    &mut DrawResources,
        stack:  &'a mut Vec<Box<dyn Widget + 'b>>,
        margin: graphics::Scalar,
        game:   &Game,
    )
        -> Result<Option<Self>, TextPanelRelatedError>
    {
        let ship_status = ShipStatus::new(
            res,
            String::new(),
            game,
        )?;
        let commands = Commands::new(
            res,
            String::new(),
        )?;

        let mut stack = Stack::new(stack, margin);

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

impl Widget for ShipControl<'_, '_> {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }

    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw(res, frame, pos)
    }
}
