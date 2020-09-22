mod anchor;
mod input;
mod traits;
mod widgets;


use winit::{
    dpi::PhysicalPosition,
    event::{
        ElementState,
        MouseButton,
    },
};

use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        window::Window,
    },
    graphics::{
        self,
        screen::Screen,
    },
    game::Game,
};

use self::{
    anchor::Anchor,
    input::Input,
    traits::{
        Draw as _,
        Size as _,
        DrawError,
    },
    widgets::{
        Diagnostics,
        Instructions,
        OrbitInfo,
        ShipControl,
        ShipInfo,
        ViewSize,
        text,
    },
};


pub struct Ui {
    input:        Input,
    scale_factor: graphics::Scalar,
}

impl Ui {
    pub fn new(window: &Window) -> Self {
        Self {
            input:        Input::new(),
            scale_factor: window.scale_factor(),
        }
    }

    pub fn handle_cursor_move(&mut self, position: PhysicalPosition<f64>) {
        self.input.cursor = Some(
            graphics::Pnt2::new(
                position.x as f32 / self.scale_factor,
                position.y as f32 / self.scale_factor,
            )
        );
    }

    pub fn handle_mouse_input(&mut self,
        state:  ElementState,
        button: MouseButton,
    ) {
        if let (MouseButton::Left, ElementState::Pressed) = (button, state) {
            self.input.click = true;
        }
    }

    pub fn handle_scale_factor_change(&mut self, scale_factor: f64) {
        self.scale_factor = scale_factor as graphics::Scalar;
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
                ::create(
                    res,
                    MARGIN,
                    game,
                    frame,
                )?
                .position(Anchor::top_left(), MARGIN, frame)
                .draw(res, frame)?;
        }

        ViewSize
            ::create(
                res,
                frame,
                game,
            )?
            .position(Anchor::bottom_left(), MARGIN, frame)
            .draw(res, frame)?;

        Instructions
            ::create(
                res,
                game,
            )?
            .position(Anchor::bottom_right(), MARGIN, frame)
            .draw(res, frame)?;

        let ship_control = ShipControl::create(
            res,
            MARGIN,
            game,
        )?;
        if let Some(ship_control) = ship_control {
            ship_control
                .position(Anchor::top_right(), MARGIN, frame)
                .process_input(&mut self.input)
                .draw(res, frame)?;
        }

        for orbit in game.state.active_orbits() {
            let orbit_info = OrbitInfo::create(
                res,
                &orbit,
                game,
                screen,
            )?;
            if let Some(mut orbit_info) = orbit_info {
                orbit_info.draw(res, frame)?;
            }
        }

        for ship in game.state.data.ships.values() {
            let ship_info = ShipInfo::create(
                res,
                ship,
                game,
                screen,
            )?;
            if let Some(mut ship_info) = ship_info {
                ship_info.draw(res, frame)?;
            }
        }

        for action in self.input.actions.drain(..) {
            println!("{:?}", action);
        }

        self.input.reset();

        Ok(())
    }
}


#[derive(Debug)]
pub enum Error {
    Text(text::CreateError),
    Draw(DrawError),
}

impl From<text::CreateError> for Error {
    fn from(err: text::CreateError) -> Self {
        Self::Text(err)
    }
}

impl From<DrawError> for Error {
    fn from(err: DrawError) -> Self {
        Self::Draw(err)
    }
}