use std::{
    collections::{
        BTreeMap,
        VecDeque,
        btree_map,
        vec_deque,
    },
    fmt,
};

use ggez::{
    Context,
    input::keyboard::is_key_repeated,
};
use time::{
    OffsetDateTime,
    Time,
};

use crate::{
    config::{
        Config,
        Key,
    },
    shared::{
        input::{
            Action,
            EventKind,
            Rotation,
        },
        math::Pnt2,
    },
    transforms::{
        Camera,
        Screen,
    },
};


pub struct Input {
    pub config: Config,

    pub pointer_screen: Screen<Pnt2>,
    pub pointer_world:  Pnt2,

    pub zoom: f32,

    pub events: Events,
}

impl Input {
    pub fn new(config: Config) -> Self {
        Self {
            config,

            pointer_screen: Screen(Pnt2::new(0.0, 0.0)),
            pointer_world:  Pnt2::new(0.0, 0.0),

            zoom: 1.0,

            events: Events::new(),
        }
    }

    pub fn mouse_motion(&mut self,
        context: &mut Context,
        x:       f32,
        y:       f32,
        camera:  &Camera,
    ) {
        self.pointer_screen.0.x = x;
        self.pointer_screen.0.y = y;

        self.pointer_world = camera.screen_to_world(
            context,
            self.pointer_screen,
        );
    }

    pub fn mouse_wheel(&mut self, y: f32) {
        self.zoom += y * 0.1;

        self.zoom = f32::min(self.zoom, 10.0);
        self.zoom = f32::max(self.zoom,  0.1);
    }

    pub fn key_down(&mut self, context: &Context, key: Key) {
        if is_key_repeated(context) {
            return;
        }

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
                self.events.push(
                    EventKind::LaunchMissile { target: self.pointer_world }
                )
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
            inner: Action {
                seq: self.next_seq,
                kind,
            },
            entered: OffsetDateTime::now().time(),
            sent:    None,
            handled: None,
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

    pub fn unsent(&mut self) -> impl Iterator<Item=Action> + '_ {
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

    pub fn handled(&mut self, seq: u64) {
        if let Some(event) = self.sent.get_mut(&seq) {
            event.handled = Some(OffsetDateTime::now().time());
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
    pub inner:   Action,
    pub entered: Time,
    pub sent:    Option<Time>,
    pub handled: Option<Time>,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "{}: {:?} ({}",
            self.inner.seq,
            self.inner.kind,
            self.entered.format("%H:%M:%S"),
        )?;
        if let Some(sent) = self.sent {
            let entered_to_sent_ms = (sent - self.entered)
                .whole_milliseconds();
            write!(f, ", sent: +{}ms", entered_to_sent_ms)?;
        }
        if let Some(handled) = self.handled {
            let entered_to_handled_ms = (handled - self.entered)
                .whole_milliseconds();
            write!(f, ", handled: +{}ms", entered_to_handled_ms)?;
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
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
            .map(|mut event| {
                event.sent = Some(OffsetDateTime::now().time());
                self.sent.insert(event.inner.seq, event);
                event.inner
            })
    }
}
