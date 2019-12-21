mod graphics;


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
};

use self::graphics::Graphics;


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
}

impl Game {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        Ok(
            Game {
                graphics: Graphics::new(context)?,
            }
        )
    }
}

impl EventHandler for Game {
    fn update(&mut self, _: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        self.graphics.draw(context)
    }
}
