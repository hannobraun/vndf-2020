mod anchor;
mod elements;


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
    elements::{
        Diagnostics,
        Element as _,
        Instructions,
        ShipStatus,
        TextPanel,
        ViewSize,
        diagnostics,
    },
};


pub use self::elements::TextPanelRelatedError as Error;


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
        let mut cache = diagnostics::Cache::new();

        let elements = ui::Elements::new(game, &frame.screen);

        const MARGIN: f32 = 20.0;

        if game.input.config.diagnostics {
            let mut stack = Vec::new();
            Diagnostics
                ::new(
                    res,
                    &mut cache,
                    &mut stack,
                    MARGIN,
                    game,
                    frame,
                )?
                .position(Anchor::top_left(), MARGIN, frame)
                .draw(res, frame);
        }

        let mut view_size_buf = String::new();
        ViewSize
            ::new(
                res,
                frame,
                &mut view_size_buf,
                game,
            )?
            .position(Anchor::bottom_left(), MARGIN, frame)
            .draw(res, frame);

        let mut instructions_buf = String::new();
        Instructions
            ::new(
                res,
                &mut instructions_buf,
                game,
            )?
            .position(Anchor::bottom_right(), MARGIN, frame)
            .draw(res, frame);

        let mut ship_status_buf = String::new();
        let ship_status = ShipStatus::new(
            res,
            &mut ship_status_buf,
            game,
        )?;
        if let Some(mut ship_status) = ship_status {
            ship_status
                .position(Anchor::top_right(), MARGIN, frame)
                .draw(res, frame);
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
}
