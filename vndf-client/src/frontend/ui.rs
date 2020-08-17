mod anchor;
mod widgets;


use winit::event::{
    Event,
    WindowEvent,
};

use crate::{
    frontend::drawers::{
        DrawResources,
        Frame,
    },
    graphics::{
        self,
        screen::Screen,
    },
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


pub struct Ui {
    pointer: Option<graphics::Pnt2>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            pointer: None,
        }
    }

    pub fn handle_event(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CursorMoved {
                    position,
                    ..
                },
                ..
            } => {
                self.pointer = Some(
                    graphics::Pnt2::new(
                        position.x as f32,
                        position.y as f32,
                    )
                );
            }
            _ => {}
        }
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

        println!("Position: {:?}", self.pointer);

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
        if let Some(ship_control) = ship_control {
            ship_control
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
