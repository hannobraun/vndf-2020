mod game;
mod graphics;
mod input;


pub use vndf_shared as shared;


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
};
use log::error;

use self::{
    game::State,
    graphics::Graphics,
    shared::net::{
        self,
        client::Conn,
        msg,
    },
};


fn main() -> Result<(), Error> {
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("vndf_shared=info,vndf_client=info")
    );

    #[cfg(feature="production")]
    let (server, conn) = {
        let server = DummyServer;
        let conn   = Conn::connect(("reineke.hannobraun.de", 34480))?;
        (server, conn)
    };

    #[cfg(not(feature = "production"))]
    let (server, conn) = {
        let server = Server::start_local()?;
        let conn   = Conn::connect(server.addr())?;
        (server, conn)
    };

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
    )
        -> Result<Self, Error>
    {
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
            self.conn.send(msg::FromClient::Input(event))
                .expect("Failed to send input event");
        }
    }

    fn key_up_event(&mut self,
        _:        &mut Context,
        key_code: KeyCode,
        _:        KeyMods,
    ) {
        if let Some(event) = input::key_up(key_code) {
            self.conn.send(msg::FromClient::Input(event))
                .expect("Failed to send input event");
        }
    }

    fn update(&mut self, context: &mut Context) -> GameResult {
        self.server.update();

        for message in self.conn.incoming() {
            match message {
                Ok(msg::FromServer::Welcome(id)) => {
                    self.state.own_id = Some(id);
                }
                Ok(msg::FromServer::UpdateEntity(entity)) => {
                    self.state.update_entity(entity);
                }
                Ok(msg::FromServer::RemoveEntity(id)) => {
                    self.state.remove_entity(id);
                }
                Err(err) => {
                    error!("Connection error: {:?}", err);
                    quit(context);
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        self.graphics.draw(context, &self.state)
    }
}


#[cfg(feature = "production")]
type Server = DummyServer;

#[cfg(not(feature = "production"))]
type Server = crate::shared::Server;


#[cfg(feature = "production")]
pub struct DummyServer;

#[cfg(feature = "production")]
impl DummyServer {
    pub fn update(&mut self) {}
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
