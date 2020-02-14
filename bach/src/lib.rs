use std::collections::VecDeque;


pub struct EventBuf<T>(VecDeque<T>);

impl<T> EventBuf<T> {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn sink(&mut self) -> EventSink<T> {
        EventSink(self)
    }

    pub fn source(&mut self) -> EventSource<T> {
        EventSource(self)
    }
}


pub struct EventSink<'r, T>(&'r mut EventBuf<T>);

impl<T> EventSink<'_, T> {
    pub fn push(&mut self, event: T) {
        (self.0).0.push_back(event);
    }
}


pub struct EventSource<'r, T>(&'r mut EventBuf<T>);

impl<T> EventSource<'_, T> {
    pub fn next(&mut self) -> Option<T> {
        (self.0).0.pop_front()
    }

    pub fn ready(&mut self) -> impl Iterator<Item=T> + '_ {
        (self.0).0.drain(..)
    }
}
