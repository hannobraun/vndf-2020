use crate::{
    camera::Camera,
    game::{
        config::{
            Config,
            Key,
        },
        coords::{
            Screen,
            World,
        },
        net::input::Events,
    },
    shared::{
        action::{
            self,
            Rotation,
        },
        math::{
            Pnt2,
            Vec2,
        },
    },
};


pub struct Handler {
    pub config: Config,

    pub pointer_screen: Screen<Pnt2>,
    pub pointer_world:  World<Pnt2>,

    pub zoom: f32,
}

impl Handler {
    pub fn new(config: Config) -> Self {
        Self {
            config,

            pointer_screen: Screen(Pnt2::new(0.0, 0.0)),
            pointer_world:  World(Pnt2::new(0.0, 0.0)),

            zoom: 1.0,
        }
    }

    pub fn handle(&mut self,
        input:       Input,
        camera:      &Camera,
        screen_size: Screen<Vec2>,
        events:      &mut Events,
    )
        -> Transition
    {
        match input {
            Input::KeyDown(key) => {
                match key {
                    k if k == self.config.input.quit => {
                        return Transition::Quit;
                    }
                    k if k == self.config.input.left => {
                        events.push(action::Kind::Rotate(Rotation::Left))
                    }
                    k if k == self.config.input.right => {
                        events.push(action::Kind::Rotate(Rotation::Right))
                    }
                    k if k == self.config.input.thrust_on => {
                        events.push(action::Kind::Thrust(true))
                    }
                    k if k == self.config.input.launch => {
                        events.push(
                            action::Kind::LaunchMissile {
                                target: self.pointer_world.0
                            }
                        )
                    }
                    _ => (),
                }
            }
            Input::KeyUp(key) => {
                match key {
                    k if k == self.config.input.left => {
                        events.push(action::Kind::Rotate(Rotation::None))
                    }
                    k if k == self.config.input.right => {
                        events.push(action::Kind::Rotate(Rotation::None))
                    }
                    k if k == self.config.input.thrust_on => {
                        events.push(action::Kind::Thrust(false))
                    }
                    _ => (),
                }
            }
            Input::MouseMotion(pos) => {
                self.pointer_screen = pos;

                self.pointer_world = camera.screen_to_world(
                    screen_size,
                    self.pointer_screen,
                );
            }
            Input::MouseWheel(y) => {
                self.zoom += y * 0.1;

                self.zoom = f32::min(self.zoom, 10.0);
                self.zoom = f32::max(self.zoom,  0.1);
            }
        }

        Transition::None
    }
}


#[derive(Debug)]
pub enum Input {
    KeyDown(Key),
    KeyUp(Key),
    MouseMotion(Screen<Pnt2>),
    MouseWheel(f32),
}

#[must_use]
#[derive(Eq, PartialEq)]
pub enum Transition {
    None,
    Quit,
}
