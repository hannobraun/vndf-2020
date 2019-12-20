use std::env;

use ggez::{
    Context,
    ContextBuilder,
    GameResult,
    conf::WindowSetup,
    event::{
        EventHandler,
        run,
    },
    graphics,
};


fn main() -> GameResult {
    // Force X11 backend to prevent panic.
    // See https://github.com/ggez/ggez/issues/579
    env::set_var("WINIT_UNIX_BACKEND", "x11");

    let (mut context, mut event_loop) =
        ContextBuilder::new("vndf", "Hanno Braun")
            .window_setup(WindowSetup {
                title: "Von Neumann Defense Force".into(),
                .. Default::default()
            })
            .build()?;

    let mut game = Game;

    run(&mut context, &mut event_loop, &mut game)
}


pub struct Game;

impl EventHandler for Game {
    fn update(&mut self, _: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, [0.0, 0.0, 0.1, 1.0].into());
        graphics::present(context)?;
        Ok(())
    }
}
