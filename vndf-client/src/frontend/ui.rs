mod elements;


use crate::{
    frontend::drawers::{
        DrawResources,
        Frame,
    },
    game::Game,
    graphics,
    ui,
};

use self::elements::{
    ComponentStats,
    Draw as _,
    FrameTime,
    InputEvents,
    Instructions,
    NetworkStats,
    Size as _,
    Stack,
    TextPanel,
    stack::StackElement,
};


pub use self::elements::TextPanelRelatedError as Error;


pub fn draw(
    res:   &mut DrawResources,
    frame: &mut Frame,
    game:  &Game,
)
    -> Result<(), Error>
{
    let mut cache = Cache::new();

    let elements = ui::Elements::new(game, &frame.screen);

    const MARGIN: f32 = 20.0;

    if game.input.config.diagnostics {
        let mut diagnostics = Stack::new(&mut cache.stack, MARGIN);

        let mut frame_time = FrameTime::new(
            res,
            &mut cache.frame_time,
            game,
        )?;
        let mut component_stats = ComponentStats::new(
            res,
            &mut cache.component_stats,
            game,
        )?;
        let mut network_stats = NetworkStats::new(
            res,
            &mut cache.network_stats,
            game,
        )?;
        let mut input_events = InputEvents::new(
            res,
            &mut cache.input_events,
            game,
        )?;

        diagnostics.add(&mut frame_time);
        diagnostics.add_iter(component_stats.as_mut().map(|e| e as _));
        diagnostics.add(&mut network_stats);
        diagnostics.add(&mut input_events);

        diagnostics.draw(res, frame, graphics::Pnt2::new(MARGIN, MARGIN));
    }

    let mut instructions_buf = String::new();
    let mut instructions = Instructions::new(
        res,
        &mut instructions_buf,
        game,
    )?;
    instructions.draw(
        res,
        frame,
        graphics::Pnt2::new(
            frame.screen.logical_size().width
                - MARGIN
                - instructions.size().width,
            frame.screen.logical_size().height
                - MARGIN
                - instructions.size().height,
        ),
    );

    let other_elements = elements.own_ship_status.iter()
        .chain(&elements.orbit_info)
        .chain(&elements.ship_info);

    for element in other_elements {
        TextPanel::new(res, &element.text)
            .unwrap()
            .draw(res, frame, element.pos);
    }

    Ok(())
}


struct Cache<'r> {
    stack: Vec<&'r mut dyn StackElement>,

    component_stats: String,
    frame_time:      String,
    input_events:    String,
    network_stats:   String,
}

impl<'r> Cache<'r> {
    fn new() -> Self {
        Self {
            stack: Vec::new(),

            component_stats: String::new(),
            frame_time:      String::new(),
            input_events:    String::new(),
            network_stats:   String::new(),
        }
    }
}
