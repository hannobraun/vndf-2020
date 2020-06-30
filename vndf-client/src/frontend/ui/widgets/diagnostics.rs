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


pub struct Diagnostics<'a>(Stack<'a>);

impl<'a> Diagnostics<'a> {
    pub fn new(
        res:    &mut DrawResources,
        stack:  &'a mut Vec<Box<dyn Widget>>,
        margin: graphics::Scalar,
        game:   &Game,
        frame:  &Frame,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let frame_time = FrameTime::new(
            res,
            game,
        )?;
        let scale_factor = ScaleFactor::new(
            res,
            frame,
        )?;
        let component_stats = ComponentStats::new(
            res,
            game,
        )?;
        let network_stats = NetworkStats::new(
            res,
            game,
        )?;
        let input_events = InputEvents::new(
            res,
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

impl Widget for Diagnostics<'_> {
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
