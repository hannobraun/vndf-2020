mod graphics;


use std::{
    convert::TryInto as _,
    env,
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
        is_key_repeated,
    },
    timer,
};
use log::{
    debug,
    error,
};

use crate::{
    game::{
        Game,
        coords::Screen,
        input::{
            Input,
            Transition,
        },
    },
    shared::{
        math::{
            Pnt2,
            Vec2,
        },
        net::{
            self,
            msg,
        },
    },
};

use self::graphics::Graphics;


pub fn start(game: Game) -> Result<(), Error> {
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
                    .fullscreen_type(FullscreenType::Windowed)
                    // This only works for me, if `resizable` is set to `true`.
                    // See https://github.com/ggez/ggez/issues/732
                    .maximized(true)
                    .resizable(true),
            )
            .build()?;

    let mut handler = Handler::new(game, &mut context)?;
    run(&mut context, &mut event_loop, &mut handler)?;

    Ok(())
}


pub struct Handler {
    game:     Game,
    graphics: Graphics,
    input:    Vec<Input>,
}

impl Handler {
    pub fn new(
        game:    Game,
        context: &mut Context,
    )
        -> Result<Self, Error>
    {
        let mut game = game;

        let color = [
            game.config.color.r,
            game.config.color.g,
            game.config.color.b,
        ];
        game.conn.send(msg::FromClient::Hello { color })?;

        Ok(
            Self {
                game,
                graphics: Graphics::new(context)?,
                input:    Vec::new(),
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
        if !is_key_repeated(context) {
            if let Ok(key) = button.try_into() {
                self.input.push(Input::KeyDown(key));
            }
        }
    }

    fn mouse_button_up_event(&mut self,
        _:      &mut Context,
        button: MouseButton,
        _x:     f32,
        _y:     f32,
    ) {
        if let Ok(key) = button.try_into() {
            self.input.push(Input::KeyUp(key));
        }
    }

    fn mouse_motion_event(&mut self,
        _:   &mut Context,
        x:   f32,
        y:   f32,
        _dx: f32,
        _dy: f32,
    ) {
        self.input.push(Input::MouseMotion(Screen(Pnt2::new(x, y))));
    }

    fn mouse_wheel_event(&mut self,
        _:  &mut Context,
        _x: f32,
        y:  f32,
    ) {
        self.input.push(Input::MouseWheel(y));
    }

    fn key_down_event(&mut self,
        context:  &mut Context,
        key_code: KeyCode,
        _:        KeyMods,
        _:        bool,
    ) {
        if !is_key_repeated(context) {
            self.input.push(Input::KeyDown(key_code.into()));
        }
    }

    fn key_up_event(&mut self,
        _:        &mut Context,
        key_code: KeyCode,
        _:        KeyMods,
    ) {
        self.input.push(Input::KeyUp(key_code.into()));
    }

    fn update(&mut self, context: &mut Context) -> GameResult {
        let (screen_width, screen_height) =
            ggez::graphics::drawable_size(context);
        let screen_size = Screen(Vec2::new(screen_width, screen_height));

        for input in self.input.drain(..) {
            let trans = self.game.handle_input(input, screen_size);

            if trans == Transition::Quit {
                quit(context);
            }
        }

        for message in self.game.conn.incoming() {
            match message {
                Ok(msg::FromServer::Ping) => {
                    // This message is just for testing purposes. Nothing to do
                    // here.
                }
                Ok(msg::FromServer::Welcome(id)) => {
                    self.game.state.own_id = Some(id);
                }
                Ok(msg::FromServer::UpdateComponent(component)) => {
                    debug!("Update component: {:?}", component);
                    self.game.state.update_component(component);
                }
                Ok(msg::FromServer::RemoveComponent(handle)) => {
                    self.game.state.remove_component(&handle);
                }
                Ok(msg::FromServer::InputHandled { seq }) => {
                    self.game.events.handled(seq);
                }
                Ok(msg::FromServer::Diagnostics(diagnostics)) => {
                    self.game.state.diagnostics = Some(diagnostics);
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
        self.game.state.frame_time.push(dt.try_into().unwrap());

        self.game.events.limit();

        let dt = timer::duration_to_f64(dt) as f32;
        self.game.state.update(dt, &self.game.input);

        self.graphics.draw(context, &self.game)
    }
}


#[derive(Debug)]
pub enum Error {
    Ggez(GameError),
    Net(net::Error),
}

impl From<GameError> for Error {
    fn from(err: GameError) -> Self {
        Self::Ggez(err)
    }
}

impl From<net::Error> for Error {
    fn from(err: net::Error) -> Self {
        Self::Net(err)
    }
}
