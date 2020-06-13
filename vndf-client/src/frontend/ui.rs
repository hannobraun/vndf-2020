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
    ShipStatus,
    Size as _,
    TextPanel,
    ViewSize,
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

    let mut view_size_buf = String::new();
    let mut view_size = ViewSize::new(
        res,
        frame,
        &mut view_size_buf,
        game,
    )?;
    view_size.draw(
        res,
        frame,
        graphics::Pnt2::new(
            MARGIN,
            frame.screen.logical_size().height
                - MARGIN
                - view_size.size().height,
        )
    );

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

    let mut ship_status_buf = String::new();
    let ship_status = ShipStatus::new(
        res,
        &mut ship_status_buf,
        game,
    )?;
    if let Some(mut ship_status) = ship_status {
        ship_status.draw(
            res,
            frame,
            graphics::Pnt2::new(
                frame.screen.logical_size().width
                    - MARGIN
                    - ship_status.size().width,
                MARGIN,
            ),
        );
    }

    let other_elements = elements.orbit_info.iter()
        .chain(&elements.ship_info);

    for element in other_elements {
        TextPanel::new(res, &element.text)
            .unwrap()
            .draw(res, frame, element.pos);
    }

    Ok(())
}
