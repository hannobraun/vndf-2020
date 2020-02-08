use std::collections::{
    VecDeque,
    vec_deque,
};

use ggez::Context;

use crate::{
    config::{
        Config,
        Key,
    },
    shared::{
        input::{
            self,
            EventKind,
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

            events: Events::new(),
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
                self.events.push(EventKind::Rotate(Rotation::Left))
            }
            k if k == self.config.input.right => {
                self.events.push(EventKind::Rotate(Rotation::Right))
            }
            k if k == self.config.input.thrust => {
                self.events.push(EventKind::Thrust(true))
            }
            k if k == self.config.input.launch => {
                if let Some(target) = self.pointer_world {
                    self.events.push(EventKind::LaunchMissile { target })
                }
            }

            _ => (),
        }
   }

    pub fn key_up(&mut self, key: Key) {
        match key {
            k if k == self.config.input.left => {
                self.events.push(EventKind::Rotate(Rotation::None))
            }
            k if k == self.config.input.right => {
                self.events.push(EventKind::Rotate(Rotation::None))
            }
            k if k == self.config.input.thrust => {
                self.events.push(EventKind::Thrust(false))
            }

            _ => (),
        }
    }
}


pub struct Events {
    unsent:   VecDeque<Event>,
    sent:     VecDeque<Event>,
    next_seq: u64,
}

impl Events {
    pub fn new() -> Self {
        Self {
            unsent:   VecDeque::new(),
            sent:     VecDeque::new(),
            next_seq: 0,
        }
    }

    pub fn push(&mut self, kind: EventKind) {
        let event = Event {
            inner: input::Event {
                seq: self.next_seq,
                kind,
            },
        };

        self.next_seq += 1;

        self.unsent.push_back(event);
    }

    pub fn iter(&self) -> Iter {
        Iter {
            unsent: self.unsent.iter(),
            sent:   self.sent.iter(),
        }
    }

    pub fn unsent(&mut self) -> impl Iterator<Item=input::Event> + '_ {
        Unsent {
            inner: self.unsent.drain(..),
            sent:  &mut self.sent,
        }
    }
}

impl<'r> IntoIterator for &'r Events {
    type IntoIter = Iter<'r>;
    type Item     = &'r Event;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Event {
    pub inner: input::Event,
}


pub struct Iter<'r> {
    unsent: vec_deque::Iter<'r, Event>,
    sent:   vec_deque::Iter<'r, Event>,
}

impl<'r> Iterator for Iter<'r> {
    type Item = &'r Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.unsent.next()
            .or_else(|| self.sent.next())
    }
}


pub struct Unsent<'r> {
    inner: vec_deque::Drain<'r, Event>,
    sent:  &'r mut VecDeque<Event>,
}

impl<'r> Iterator for Unsent<'r> {
    type Item = input::Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
            .map(|event| {
                self.sent.push_back(event);
                event.inner
            })
    }
}
