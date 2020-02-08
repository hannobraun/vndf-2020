use std::collections::VecDeque;

use ggez::Context;

use crate::{
    config::{
        Config,
        Key,
    },
    shared::{
        input::{
            Event,
            Rotation,
        },
        math::Pnt2,
    },
    transforms,
};


pub struct Input {
    pub config: Config,

    pub pointer_screen: Pnt2,
    pub pointer_world:  Option<Pnt2>,

    pub events: Events,
}

impl Input {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            pointer_screen: Pnt2::new(0.0, 0.0),
            pointer_world:  None,

            events: Events(VecDeque::new()),
        }
    }

    pub fn mouse_motion(&mut self, context: &mut Context, x: f32, y: f32) {
        self.pointer_screen.x = x;
        self.pointer_screen.y = y;

        self.pointer_world = transforms::screen_to_world(
            context,
            self.pointer_screen,
        );
    }

    pub fn key_down(&mut self, key: Key) {
        match key {
            k if k == self.config.input.left => {
                self.events.push(Event::Rotate(Rotation::Left))
            }
            k if k == self.config.input.right => {
                self.events.push(Event::Rotate(Rotation::Right))
            }
            k if k == self.config.input.thrust => {
                self.events.push(Event::Thrust(true))
            }
            k if k == self.config.input.launch => {
                if let Some(target) = self.pointer_world {
                    self.events.push(Event::LaunchMissile { target })
                }
            }

            _ => (),
        }
   }

    pub fn key_up(&mut self, key: Key) {
        match key {
            k if k == self.config.input.left => {
                self.events.push(Event::Rotate(Rotation::None))
            }
            k if k == self.config.input.right => {
                self.events.push(Event::Rotate(Rotation::None))
            }
            k if k == self.config.input.thrust => {
                self.events.push(Event::Thrust(false))
            }

            _ => (),
        }
    }
}


pub struct Events(pub VecDeque<Event>);

impl Events {
    pub fn push(&mut self, event: Event) {
        self.0.push_front(event);
    }

    pub fn drain(&mut self) -> impl Iterator<Item=Event> + '_ {
        self.0.drain(..)
    }
}
