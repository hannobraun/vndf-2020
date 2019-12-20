use std::env;

use ggez::{
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
    graphics::{
        self,
        DrawMode,
        DrawParam,
        Mesh,
    },
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
            .window_mode(WindowMode {
                fullscreen_type: FullscreenType::Desktop,
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

        let ship = Mesh::new_polygon(
            context,
            DrawMode::fill(),
            &[
                [ 100.0,    0.0],
                [-100.0,  100.0],
                [-100.0, -100.0],
            ],
            [1.0, 1.0, 0.0, 1.0].into(),
        )?;
        graphics::draw(
            context,
            &ship,
            DrawParam::new()
                .dest([300.0, 300.0]),
        )?;

        graphics::present(context)?;
        Ok(())
    }
}
