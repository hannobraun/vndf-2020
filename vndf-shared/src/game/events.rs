use std::collections::VecDeque;

use crate::game::entities::Missile;


pub struct Events(VecDeque<Event>);

impl Events {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self, event: Event) {
        self.0.push_back(event);
    }

    pub fn drain(&mut self) -> impl Iterator<Item=Event> + '_ {
        self.0.drain(..)
    }
}


pub enum Event {
    LaunchMissile(Missile),
}
