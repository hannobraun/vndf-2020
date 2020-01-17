use std::collections::VecDeque;

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
}


pub struct Push<'r, T>(pub &'r mut VecDeque<T>);


macro_rules! events {
    (
        $(
            $event:ident, $event_lower:ident {
                $($arg_name:ident: $arg_type:ty,)*
            }
        )*
    ) => {
        use crate::events::Push;

        impl Push<'_, Event> {
            $(
                pub fn $event_lower(&mut self, $($arg_name: $arg_type,)*) {
                    self.0.push_back(Event::$event { $($arg_name,)* });
                }
            )*
        }


        pub enum Event {
            $(
                $event {
                    $($arg_name: $arg_type,)*
                },
            )*
        }
    };
}
