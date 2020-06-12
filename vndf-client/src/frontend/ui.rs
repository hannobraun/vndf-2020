mod elements;
mod layout;


use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
    },
    game::Game,
    graphics,
    ui,
};

use self::{
    elements::{
        Element as _,
        FrameTime,
        Instructions,
        TextPanel,
    },
    layout::Layout,
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

    let mut top_left = Layout::new(
        graphics::Pnt2::new(MARGIN, MARGIN),
        MARGIN,
    );

    let instructions =
        Instructions::new(
            res,
            &mut cache.instructions,
            game,
        )?;
    let frame_time =
        FrameTime::new(
            res,
            &mut cache.frame_time,
            game,
        )?;

    top_left.draw(res, frame, instructions);
    top_left.draw_iter(res, frame, frame_time);
    if let Some(element) = elements.diagnostics.as_ref() {
        top_left.draw_legacy_element(res, frame, element);
    }
    if let Some(element) = elements.input_events.as_ref() {
        top_left.draw_legacy_element(res, frame, element);
    }

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
    frame_time,
    instructions,
);
