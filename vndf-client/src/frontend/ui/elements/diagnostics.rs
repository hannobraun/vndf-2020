use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::elements::Widget,
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
        cache:  &'b mut Cache,
        stack:  &'a mut Vec<Box<dyn Widget + 'b>>,
        margin: graphics::Scalar,
        game:   &Game,
        frame:  &Frame,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let frame_time = FrameTime::new(
            res,
            &mut cache.frame_time,
            game,
        )?;
        let scale_factor = ScaleFactor::new(
            res,
            &mut cache.scale_factor,
            frame,
        )?;
        let component_stats = ComponentStats::new(
            res,
            &mut cache.component_stats,
            game,
        )?;
        let network_stats = NetworkStats::new(
            res,
            &mut cache.network_stats,
            game,
        )?;
        let input_events = InputEvents::new(
            res,
            &mut cache.input_events,
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


pub struct Cache {
    component_stats: String,
    frame_time:      String,
    input_events:    String,
    network_stats:   String,
    scale_factor:    String,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            component_stats: String::new(),
            frame_time:      String::new(),
            input_events:    String::new(),
            network_stats:   String::new(),
            scale_factor:    String::new(),
        }
    }
}
