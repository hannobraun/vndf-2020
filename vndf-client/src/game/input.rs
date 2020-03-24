use crate::{
    camera::Camera,
    game::{
        config::{
            Config,
            Key,
        },
        net::input::Events,
        transforms::{
            Screen,
            World,
        },
    },
    shared::{
        input::{
            EventKind,
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
    ) {
        match input {
            Input::MouseMotion(pos) => {
                self.pointer_screen = pos;

                self.pointer_world = camera.screen_to_world(
                    screen_size,
                    self.pointer_screen,
                );
            }
        }
    }

    pub fn mouse_wheel(&mut self, y: f32) {
        self.zoom += y * 0.1;

        self.zoom = f32::min(self.zoom, 10.0);
        self.zoom = f32::max(self.zoom,  0.1);
    }

    pub fn key_down(&mut self, key: Key, events: &mut Events) {
        match key {
            k if k == self.config.input.left => {
                events.push(EventKind::Rotate(Rotation::Left))
            }
            k if k == self.config.input.right => {
                events.push(EventKind::Rotate(Rotation::Right))
            }
            k if k == self.config.input.thrust => {
                events.push(EventKind::Thrust(true))
            }
            k if k == self.config.input.launch => {
                events.push(
                    EventKind::LaunchMissile { target: self.pointer_world.0 }
                )
            }

            _ => (),
        }
   }

    pub fn key_up(&mut self, key: Key, events: &mut Events) {
        match key {
            k if k == self.config.input.left => {
                events.push(EventKind::Rotate(Rotation::None))
            }
            k if k == self.config.input.right => {
                events.push(EventKind::Rotate(Rotation::None))
            }
            k if k == self.config.input.thrust => {
                events.push(EventKind::Thrust(false))
            }

            _ => (),
        }
    }
}


pub enum Input {
    MouseMotion(Screen<Pnt2>),
}
