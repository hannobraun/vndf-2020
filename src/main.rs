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
        Rect,
    },
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


pub struct Game;

impl Game {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let (width, height) = graphics::drawable_size(context);
        let aspect_ratio = width / height;

        let min_size = 1000.0;

        let size = if aspect_ratio >= 1.0 {
            [min_size * aspect_ratio, min_size]
        }
        else {
            [min_size, min_size / aspect_ratio]
        };

        let screen_coordinates = Rect {
            x: -size[0] / 2.0,
            y: -size[1] / 2.0,
            w: size[0],
            h: size[1],
        };

        graphics::set_screen_coordinates(context, screen_coordinates)?;

        Ok(Game)
    }
}

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
                [ 0.5,  0.0],
                [-0.5,  0.4],
                [-0.2,  0.0],
                [-0.5, -0.4],
            ],
            [1.0, 1.0, 0.0, 1.0].into(),
        )?;
        graphics::draw(
            context,
            &ship,
            DrawParam::new()
                .dest([0.0, 0.0])
                .scale([50.0, 50.0]),
        )?;

        graphics::present(context)?;
        Ok(())
    }
}
