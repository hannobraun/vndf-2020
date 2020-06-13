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
    Diagnostics,
    Draw as _,
    Instructions,
    Size as _,
    TextPanel,
    diagnostics,
};


pub use self::elements::TextPanelRelatedError as Error;


pub fn draw(
    res:   &mut DrawResources,
    frame: &mut Frame,
    game:  &Game,
)
    -> Result<(), Error>
{
    let mut cache = diagnostics::Cache::new();

    let elements = ui::Elements::new(game, &frame.screen);

    const MARGIN: f32 = 20.0;

    if game.input.config.diagnostics {
        let mut stack = Vec::new();
        let mut diagnostics = Diagnostics::new(
            res,
            &mut cache,
            &mut stack,
            MARGIN,
            game,
            frame,
        )?;
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
