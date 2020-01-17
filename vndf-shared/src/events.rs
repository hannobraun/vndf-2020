use std::{
    collections::VecDeque,
    iter,
};

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


        pub enum $name {
            $(
                $event {
                    $($arg_name: $arg_type,)*
                },
            )*
        }
    };
}
