mod graphics;
mod input;
mod math;
mod state;


use std::{
    env,
    io,
};

use ggez::{
    self,
    Context,
    ContextBuilder,
    GameError,
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
use log::error;

use vndf_shared::{
    Server,
    net::{
        self,
        client::Conn,
        msg,
    },
};

use self::{
    graphics::Graphics,
    state::State,
};


fn main() -> Result<(), Error> {
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("vndf_shared=info,vndf_client=info")
    );

    let server = Server::start_local()?;
    let conn   = Conn::connect(server.addr())?;

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

    let mut game = Game::new(server, conn, &mut context)?;

    run(&mut context, &mut event_loop, &mut game)?;
    Ok(())
}


const TARGET_FPS: u32 = 60;
const FRAME_TIME: f32 = 1.0 / TARGET_FPS as f32;


pub struct Game {
    server:   Server,
    conn:     Conn,
    graphics: Graphics,
    state:    State,
}

impl Game {
    pub fn new(
        server:  Server,
        conn:    Conn,
        context: &mut Context,
    ) -> Result<Self, Error> {
        let mut conn = conn;
        conn.send(msg::FromClient::Hello)?;

        Ok(
            Game {
                server,
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
        self.server.update();

        for message in self.conn.incoming() {
            match message {
                Ok(message) => {
                    print!("Message: {:?}\n", message)
                }
                Err(err) => {
                    error!("Connection error: {:?}", err);
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


#[derive(Debug)]
pub enum Error {
    Ggez(GameError),
    Io(io::Error),
    Net(net::Error),
}

impl From<GameError> for Error {
    fn from(err: GameError) -> Self {
        Self::Ggez(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<net::Error> for Error {
    fn from(err: net::Error) -> Self {
        Self::Net(err)
    }
}
