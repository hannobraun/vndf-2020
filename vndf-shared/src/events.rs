use std::{
    collections::VecDeque,
    iter,
};


pub struct Stream<T>(VecDeque<T>);

impl<T> Stream<T> {
    pub fn new() -> Self {
        Stream(VecDeque::new())
    }

    pub fn sink(&mut self) -> Sink<T> {
        Sink(self)
    }

    pub fn source(&mut self) -> Source<T> {
        Source(self)
    }
}


pub struct Sink<'r, T>(&'r mut Stream<T>);

impl<T> Sink<'_, T> {
    pub fn push(&mut self, event: T) {
        (self.0).0.push_back(event);
    }
}


pub struct Source<'r, T>(&'r mut Stream<T>);

impl<T> Source<'_, T> {
    pub fn ready(&mut self) -> impl Iterator<Item=T> + '_ {
        (self.0).0.drain(..)
    }
}


pub struct Events<T>(VecDeque<T>);

impl<T> Events<T> {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self) -> Push<T> {
        Push(&mut self.0)
    }

    pub fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    pub fn drain(&mut self) -> impl Iterator<Item=T> + '_ {
        iter::from_fn(move || self.next())
    }
}


pub struct Push<'r, T>(pub &'r mut VecDeque<T>);


macro_rules! events {
    (
        $name:ident {
            $(
                $event:ident, $event_lower:ident {
                    $($arg_name:ident: $arg_type:ty,)*
                }
            )*
        }
    ) => {
        use crate::events::Push;

        impl Push<'_, $name> {
            $(
                pub fn $event_lower(&mut self, $($arg_name: $arg_type,)*) {
                    self.0.push_back($name::$event { $($arg_name,)* });
                }
            )*
        }


        #[derive(Debug)]
        pub enum $name {
            $(
                $event {
                    $($arg_name: $arg_type,)*
                },
            )*
        }
    };
}
