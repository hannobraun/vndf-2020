mod anchor;
mod widgets;


use crate::{
    frontend::drawers::{
        DrawResources,
        Frame,
    },
    game::Game,
    ui,
};

use self::{
    anchor::Anchor,
    widgets::{
        Diagnostics,
        Instructions,
        ShipControl,
        TextPanel,
        ViewSize,
        Widget as _,
    },
};


pub use self::widgets::TextPanelRelatedError as Error;


pub struct Ui {}

impl Ui {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        game:  &Game,
    )
        -> Result<(), Error>
    {
        let elements = ui::Elements::new(game, &frame.screen);

        const MARGIN: f32 = 20.0;

        if game.input.config.diagnostics {
            let stack = Vec::new();
            Diagnostics
                ::new(
                    res,
                    stack,
                    MARGIN,
                    game,
                    frame,
                )?
                .position(Anchor::top_left(), MARGIN, frame)
                .draw(res, frame);
        }

        ViewSize
            ::new(
                res,
                frame,
                game,
            )?
            .position(Anchor::bottom_left(), MARGIN, frame)
            .draw(res, frame);

        Instructions
            ::new(
                res,
                game,
            )?
            .position(Anchor::bottom_right(), MARGIN, frame)
            .draw(res, frame);

        let stack = Vec::new();
        let ship_control = ShipControl::new(
            res,
            stack,
            MARGIN,
            game,
        )?;
        if let Some(mut ship_status) = ship_control {
            ship_status
                .position(Anchor::top_right(), MARGIN, frame)
                .draw(res, frame);
        }

        let other_elements = elements.orbit_info.into_iter()
            .chain(elements.ship_info);

        for element in other_elements {
            TextPanel::new(res, element.text)
                .unwrap()
                .draw(res, frame, element.pos);
        }

        Ok(())
    }
}
