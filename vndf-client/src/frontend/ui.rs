mod element;
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

use self::layout::Layout;


pub fn draw(
    res:   &mut DrawResources,
    frame: &mut Frame,
    game:  &Game,
)
    -> Result<(), ()>
{
    let elements = ui::Elements::new(game, &frame.screen);

    const MARGIN: f32 = 20.0;

    let mut top_left = Layout::new(
        res,
        frame,
        graphics::Pnt2::new(MARGIN, MARGIN),
        MARGIN,
    );

    top_left.draw(&elements.instructions);
    if let Some(element) = elements.frame_time.as_ref() {
        top_left.draw(element);
    }
    if let Some(element) = elements.diagnostics.as_ref() {
        top_left.draw(element);
    }
    if let Some(element) = elements.input_events.as_ref() {
        top_left.draw(element);
    }

    let other_elements = elements.own_ship_status.iter()
        .chain(&elements.orbit_info)
        .chain(&elements.ship_info);

    for element in other_elements {
        element::draw(
            res,
            frame,
            element.pos,
            element.text.as_str(),
        );
    }

    Ok(())
}
