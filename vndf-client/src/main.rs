mod graphics;
mod input;
mod math;
mod net;
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
        quit,
        run,
    },
    input::keyboard::{
        KeyCode,
        KeyMods,
    },
    timer,
};

use vndf_shared::net::Server;

use self::{
    graphics::Graphics,
    net::{
        Conn,
        ReceiveError,
    },
    state::State,
};


fn main() -> GameResult {
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("vndf_shared=info,vndf_client=info")
    );

    let _server = Server::start_default()?;
    let conn    = Conn::connect()?;

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

    let mut game = Game::new(conn, &mut context)?;

    run(&mut context, &mut event_loop, &mut game)
}


const TARGET_FPS: u32 = 60;
const FRAME_TIME: f32 = 1.0 / TARGET_FPS as f32;


pub struct Game {
    conn:     Conn,
    graphics: Graphics,
    state:    State,
}

impl Game {
    pub fn new(conn: Conn, context: &mut Context) -> GameResult<Self> {
        Ok(
            Game {
                conn,
                graphics: Graphics::new(context)?,
                state:    State::new(),
            }
        )
    }
}

impl EventHandler for Game {
    fn key_down_event(&mut self,
        context:  &mut Context,
        key_code: KeyCode,
        _:        KeyMods,
        _:        bool,
    ) {
        if key_code == KeyCode::Escape {
            quit(context);
        }

        if let Some(event) = input::key_down(key_code) {
            self.state.handle_input(event);
        }
    }

    fn key_up_event(&mut self,
        _:        &mut Context,
        key_code: KeyCode,
        _:        KeyMods,
    ) {
        if let Some(event) = input::key_up(key_code) {
            self.state.handle_input(event);
        }
    }

    fn update(&mut self, context: &mut Context) -> GameResult {
        for message in self.conn.messages() {
            match message {
                Ok(message) => {
                    print!("Message: {:?}\n", message)
                }
                Err(ReceiveError) => {
                    // Error message has already been logged by the receiver
                    // thread.
                    quit(context);
                    return Ok(());
                }
            }
        }

        while timer::check_update_time(context, TARGET_FPS) {
            self.state.update(FRAME_TIME);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        self.graphics.draw(context, &self.state)
    }
}
