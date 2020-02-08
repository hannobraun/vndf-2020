use std::{
    collections::{
        BTreeMap,
        VecDeque,
        btree_map,
        vec_deque,
    },
    fmt,
};

use ggez::Context;
use time::Time;

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
    sent:     BTreeMap<u64, Event>,
    next_seq: u64,
}

impl Events {
    pub fn new() -> Self {
        Self {
            unsent:   VecDeque::new(),
            sent:     BTreeMap::new(),
            next_seq: 0,
        }
    }

    pub fn push(&mut self, kind: EventKind) {
        let event = Event {
            inner: input::Event {
                seq: self.next_seq,
                kind,
            },
            entered: Time::now(),
            sent:    None,
        };

        self.next_seq += 1;

        self.unsent.push_back(event);
    }

    pub fn iter(&self) -> Iter {
        Iter {
            unsent: self.unsent.iter(),
            sent:   self.sent.values(),
        }
    }

    pub fn unsent(&mut self) -> impl Iterator<Item=input::Event> + '_ {
        Unsent {
            inner: self.unsent.drain(..),
            sent:  &mut self.sent,
        }
    }

    pub fn limit(&mut self) {
        while self.unsent.len() + self.sent.len() > 10 {
            let first = self.sent.keys().copied().next();
            if let Some(first) = first {
                self.sent.remove(&first);
            }
            else {
                break;
            }
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
    pub inner:   input::Event,
    pub entered: Time,
    pub sent:    Option<Time>,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let time_fmt = "%H:%M:%S";

        write!(f, "{:?} ({}", self.inner, self.entered.format(time_fmt))?;
        if let Some(sent) = self.sent {
            write!(f, ", {}", sent.format(time_fmt))?;
        }
        write!(f, ")")?;

        Ok(())
    }
}


pub struct Iter<'r> {
    unsent: vec_deque::Iter<'r, Event>,
    sent:   btree_map::Values<'r, u64, Event>,
}

impl<'r> Iterator for Iter<'r> {
    type Item = &'r Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.unsent.next()
            .or_else(|| self.sent.next())
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.sent.next_back()
            .or_else(|| self.unsent.next_back())
    }
}


pub struct Unsent<'r> {
    inner: vec_deque::Drain<'r, Event>,
    sent:  &'r mut BTreeMap<u64, Event>,
}

impl<'r> Iterator for Unsent<'r> {
    type Item = input::Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
            .map(|mut event| {
                event.sent = Some(Time::now());
                self.sent.insert(event.inner.seq, event);
                event.inner
            })
    }
}
