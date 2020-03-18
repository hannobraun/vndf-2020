use std::{
    convert::TryInto as _,
    env,
    io,
    net::ToSocketAddrs,
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
        MouseButton,
        quit,
        run,
    },
    input::keyboard::{
        KeyCode,
        KeyMods,
    },
    timer,
};
use log::{
    debug,
    error,
};

use crate::{
    game::{
        config::{
            self,
            Config,
            Key,
        },
        input::Input,
        state::State,
    },
    graphics::Graphics,
    shared::net::{
        self,
        client::Conn,
        msg,
    },
};


pub fn start<A: ToSocketAddrs>(addr: A) -> Result<(), Error> {
    // Force X11 backend to prevent panic.
    // See https://github.com/ggez/ggez/issues/579
    env::set_var("WINIT_UNIX_BACKEND", "x11");

    let config = Config::load()?;

    let (mut context, mut event_loop) =
        ContextBuilder::new("vndf", "Hanno Braun")
            .window_setup(
                WindowSetup::default()
                    .title("Von Neumann Defense Force")
            )
            .window_mode(
                WindowMode::default()
                    .fullscreen_type(FullscreenType::Windowed)
                    // This only works for me, if `resizable` is set to `true`.
                    // See https://github.com/ggez/ggez/issues/732
                    .maximized(true)
                    .resizable(true),
            )
            .build()?;

    let input = Input::new(config);

    let     conn = Conn::connect(addr)?;
    let mut game = Handler::new(conn, input, config, &mut context)?;

    run(&mut context, &mut event_loop, &mut game)?;

    Ok(())
}


pub struct Handler {
    conn:     Conn,
    graphics: Graphics,
    input:    Input,
    state:    State,
}

impl Handler {
    pub fn new(
        conn:    Conn,
        input:   Input,
        config:  Config,
        context: &mut Context,
    )
        -> Result<Self, Error>
    {
        let mut conn = conn;

        let color = [
            config.color.r,
            config.color.g,
            config.color.b,
        ];
        conn.send(msg::FromClient::Hello { color })?;

        Ok(
            Self {
                conn,
                input,
                graphics: Graphics::new(context)?,
                state:    State::new(),
            }
        )
    }
}

impl EventHandler for Handler {
    fn mouse_button_down_event(&mut self,
        context: &mut Context,
        button:  MouseButton,
        _x:      f32,
        _y:      f32,
    ) {
        self.input.key_down(context, Key::Mouse(button));
    }

    fn mouse_button_up_event(&mut self,
        _:      &mut Context,
        button: MouseButton,
        _x:     f32,
        _y:     f32,
    ) {
        self.input.key_up(Key::Mouse(button));
    }

    fn mouse_motion_event(&mut self,
        context: &mut Context,
        x:       f32,
        y:       f32,
        _dx:     f32,
        _dy:     f32,
    ) {
        self.input.mouse_motion(context, x, y, &self.state.camera);
    }

    fn mouse_wheel_event(&mut self,
        _:  &mut Context,
        _x: f32,
        y:  f32,
    ) {
        self.input.mouse_wheel(y);
    }

    fn key_down_event(&mut self,
        context:  &mut Context,
        key_code: KeyCode,
        _:        KeyMods,
        _:        bool,
    ) {
        if key_code == KeyCode::Escape {
            quit(context);
        }

        self.input.key_down(context, Key::Keyboard(key_code));
    }

    fn key_up_event(&mut self,
        _:        &mut Context,
        key_code: KeyCode,
        _:        KeyMods,
    ) {
        self.input.key_up(Key::Keyboard(key_code));
    }

    fn update(&mut self, context: &mut Context) -> GameResult {
        for event in self.input.events.unsent() {
            self.conn.send(msg::FromClient::Action(event))
                .expect("Failed to send input event");
        }

        for message in self.conn.incoming() {
            match message {
                Ok(msg::FromServer::Ping) => {
                    // This message is just for testing purposes. Nothing to do
                    // here.
                }
                Ok(msg::FromServer::Welcome(id)) => {
                    self.state.own_id = Some(id);
                }
                Ok(msg::FromServer::UpdateComponent(component)) => {
                    debug!("Update component: {:?}", component);
                    self.state.update_component(component);
                }
                Ok(msg::FromServer::RemoveComponent(handle)) => {
                    self.state.remove_component(&handle);
                }
                Ok(msg::FromServer::InputHandled { seq }) => {
                    self.input.events.handled(seq);
                }
                Ok(msg::FromServer::Diagnostics(diagnostics)) => {
                    self.state.diagnostics = Some(diagnostics);
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
        let dt = timer::delta(context);
        self.state.frame_time.push(dt.try_into().unwrap());

        self.input.events.limit();

        let dt = timer::duration_to_f64(dt) as f32;
        self.state.update(dt, &self.input);

        self.graphics.draw(context, &self.input, &self.state)
    }
}


#[derive(Debug)]
pub enum Error {
    Config(config::Error),
    Ggez(GameError),
    Io(io::Error),
    Net(net::Error),
}

impl From<config::Error> for Error {
    fn from(err: config::Error) -> Self {
        Self::Config(err)
    }
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
