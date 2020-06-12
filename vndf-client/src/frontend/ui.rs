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

    let mut top_left = Stack::new(MARGIN);

    let mut frame_time = FrameTime::new(
        res,
        &mut cache.frame_time,
        game,
    )?;
    let mut component_stats = ComponentStats::new(
        res,
        &mut cache.diagnostics,
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

    top_left.add_iter(frame_time.as_mut().map(|e| e as _));
    top_left.add_iter(component_stats.as_mut().map(|e| e as _));
    top_left.add_iter(network_stats.as_mut().map(|e| e as _));
    top_left.add_iter(input_events.as_mut().map(|e| e as _));

    top_left.draw(res, frame, graphics::Pnt2::new(MARGIN, MARGIN));

    let mut instructions = Instructions::new(
        res,
        &mut cache.instructions,
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


macro_rules! cache {
    ($($entry:ident,)*) => {
        struct Cache {
            $($entry: String,)*
        }

        impl Cache {
            fn new() -> Self {
                Self {
                    $($entry: String::new(),)*
                }
            }
        }
    };
}

cache!(
    diagnostics,
    frame_time,
    input_events,
    instructions,
    network_stats,
);
