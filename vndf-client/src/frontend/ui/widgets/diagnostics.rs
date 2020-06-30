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
    ComponentStats,
    FrameTime,
    InputEvents,
    NetworkStats,
    ScaleFactor,
    Stack,
    TextPanelRelatedError,
};


pub struct Diagnostics<'a, 'b>(Stack<'a, 'b>);

impl<'a, 'b> Diagnostics<'a, 'b> {
    pub fn new(
        res:    &mut DrawResources,
        stack:  &'a mut Vec<Box<dyn Widget + 'b>>,
        margin: graphics::Scalar,
        game:   &Game,
        frame:  &Frame,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let frame_time = FrameTime::new(
            res,
            String::new(),
            game,
        )?;
        let scale_factor = ScaleFactor::new(
            res,
            String::new(),
            frame,
        )?;
        let component_stats = ComponentStats::new(
            res,
            String::new(),
            game,
        )?;
        let network_stats = NetworkStats::new(
            res,
            String::new(),
            game,
        )?;
        let input_events = InputEvents::new(
            res,
            String::new(),
            game,
        )?;

        let mut diagnostics = Stack::new(stack, margin);

        diagnostics.add(frame_time);
        diagnostics.add(scale_factor);
        diagnostics.add_iter(component_stats);
        diagnostics.add(network_stats);
        diagnostics.add(input_events);

        Ok(
            Self(diagnostics)
        )
    }
}

impl Widget for Diagnostics<'_, '_> {
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
