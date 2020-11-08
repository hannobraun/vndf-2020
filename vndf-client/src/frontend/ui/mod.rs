mod anchor;
mod input;
mod traits;
mod widgets;

use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton},
};

use crate::{
    frontend::{
        drawers::{DrawResources, Frame},
        window::Window,
    },
    game::{self, Game},
    graphics::{self, screen::Screen},
};

use self::{
    anchor::Anchor,
    input::{Action, Input},
    traits::{Draw as _, DrawError, ProcessInputAt as _},
    widgets::{
        text, Canvas, Diagnostics, Instructions, OrbitInfo, ShipControl,
        ShipInfo, ViewSize,
    },
};

pub struct Ui {
    input: Input,
    scale_factor: graphics::Scalar,
}

impl Ui {
    pub fn new(window: &Window) -> Self {
        Self {
            input: Input::new(),
            scale_factor: window.scale_factor(),
        }
    }

    pub fn handle_cursor_move(&mut self, position: PhysicalPosition<f64>) {
        self.input.cursor = Some(graphics::Pnt2::new(
            position.x as f32 / self.scale_factor,
            position.y as f32 / self.scale_factor,
        ));
    }

    pub fn handle_mouse_input(
        &mut self,
        state: ElementState,
        button: MouseButton,
    ) {
        if let (MouseButton::Left, ElementState::Pressed) = (button, state) {
            self.input.click = true;
        }
    }

    pub fn handle_scale_factor_change(&mut self, scale_factor: f64) {
        self.scale_factor = scale_factor as graphics::Scalar;
    }

    pub fn draw(
        &mut self,
        res: &mut DrawResources,
        frame: &mut Frame,
        game: &mut Game,
        screen: &Screen,
    ) -> Result<(), Error> {
        const MARGIN: f32 = 20.0;

        let mut canvas = Canvas::create(MARGIN);

        if game.input.config.diagnostics {
            canvas.add_anchored(
                Diagnostics::create(res, MARGIN, game, frame)?,
                Anchor::top_left(),
                frame,
            );
        }

        canvas.add_anchored(
            ViewSize::create(res, frame, game)?,
            Anchor::bottom_left(),
            frame,
        );

        canvas.add_anchored(
            Instructions::create(res, game)?,
            Anchor::bottom_right(),
            frame,
        );

        let ship_control = ShipControl::create(res, MARGIN, game)?;
        if let Some(ship_control) = ship_control {
            canvas.add_anchored(ship_control, Anchor::top_right(), frame);
        }

        for orbit in game.state.active_orbits() {
            let orbit_info = OrbitInfo::create(res, &orbit, game, screen)?;
            if let Some(orbit_info) = orbit_info {
                canvas.add_at(orbit_info, graphics::Pnt2::zero());
            }
        }

        for ship in game.state.data.ships.values() {
            let ship_info = ShipInfo::create(res, ship, game, screen)?;
            if let Some(ship_info) = ship_info {
                canvas.add_at(ship_info, graphics::Pnt2::zero());
            }
        }

        canvas.process_input_at(&mut self.input, graphics::Pnt2::zero());
        canvas.draw(res, frame)?;

        for action in self.input.actions.drain(..) {
            match action {
                Action::AddCommand => {
                    game.state.add_command();
                }
                Action::FtlJump => {
                    let _ = game.handle_input(game::Input::FtlJump);
                }
            }
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
