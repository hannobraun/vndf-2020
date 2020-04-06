use std::{
    collections::{
        BTreeMap,
        VecDeque,
        btree_map,
        vec_deque,
    },
    fmt,
};

use time::{
    OffsetDateTime,
    Time,
};

use crate::shared::action::{
    Action,
    EventKind,
};


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
