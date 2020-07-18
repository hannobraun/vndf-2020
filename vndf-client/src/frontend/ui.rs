mod anchor;
mod widgets;


use crate::{
    frontend::drawers::{
        DrawResources,
        Frame,
    },
    graphics::screen::Screen,
    game::Game,
};

use self::{
    anchor::Anchor,
    widgets::{
        Diagnostics,
        Draw as _,
        Instructions,
        OrbitInfo,
        ShipControl,
        ShipInfo,
        Size as _,
        ViewSize,
    },
};


pub use self::widgets::TextPanelRelatedError as Error;


pub struct Ui;

impl Ui {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&mut self,
        res:    &mut DrawResources,
        frame:  &mut Frame,
        game:   &Game,
        screen: &Screen,
    )
        -> Result<(), Error>
    {
        const MARGIN: f32 = 20.0;

        if game.input.config.diagnostics {
            Diagnostics
                ::new(
                    res,
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

        let ship_control = ShipControl::new(
            res,
            MARGIN,
            game,
        )?;
        if let Some(ship_status) = ship_control {
            ship_status
                .position(Anchor::top_right(), MARGIN, frame)
                .draw(res, frame);
        }

        for orbit in game.state.active_orbits() {
            let orbit_info = OrbitInfo::new(
                res,
                &orbit,
                game,
                screen,
            )?;
            if let Some(mut orbit_info) = orbit_info {
                orbit_info.draw(res, frame);
            }
        }

        for ship in game.state.data.ships.values() {
            let ship_info = ShipInfo::new(
                res,
                ship,
                game,
                screen,
            )?;
            if let Some(mut ship_info) = ship_info {
                ship_info.draw(res, frame);
            }
        }

        Ok(())
    }
}
