use std::collections::VecDeque;


pub struct EventBuf<T>(VecDeque<T>);

impl<T> EventBuf<T> {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn sink(&mut self) -> Sink<T> {
        Sink(self)
    }

    pub fn source(&mut self) -> Source<T> {
        Source(self)
    }
}


pub struct Sink<'r, T>(&'r mut EventBuf<T>);

impl<T> Sink<'_, T> {
    pub fn push(&mut self, event: T) {
        (self.0).0.push_back(event);
    }
}


pub struct Source<'r, T>(&'r mut EventBuf<T>);

impl<T> Source<'_, T> {
    pub fn next(&mut self) -> Option<T> {
        (self.0).0.pop_front()
    }

    pub fn ready(&mut self) -> impl Iterator<Item=T> + '_ {
        (self.0).0.drain(..)
    }
}
