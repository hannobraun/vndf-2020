mod graphics;
mod math;
mod state;


use std::env;

use ggez::{
    self,
    Context,
    ContextBuilder,
    GameResult,
    conf::{
        FullscreenType,
        WindowMode,
        WindowSetup,
    },
    event::{
        EventHandler,
        run,
    },
    timer,
};

use self::{
    graphics::Graphics,
    state::State,
};


fn main() -> GameResult {
    // Force X11 backend to prevent panic.
    // See https://github.com/ggez/ggez/issues/579
    env::set_var("WINIT_UNIX_BACKEND", "x11");

    let (mut context, mut event_loop) =
        ContextBuilder::new("vndf", "Hanno Braun")
            .window_setup(
                WindowSetup::default()
                    .title("Von Neumann Defense Force")
            )
            .window_mode(
                WindowMode::default()
                    .fullscreen_type(FullscreenType::Desktop),
            )
            .build()?;

    let mut game = Game::new(&mut context)?;

    run(&mut context, &mut event_loop, &mut game)
}


pub struct Game {
    graphics: Graphics,
    state:    State,
}

impl Game {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        Ok(
            Game {
                graphics: Graphics::new(context)?,
                state:    State::new(),
            }
        )
    }
}

impl EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while timer::check_update_time(context, 60) {
            self.state.update();
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        self.graphics.draw(context, &self.state)
    }
}
