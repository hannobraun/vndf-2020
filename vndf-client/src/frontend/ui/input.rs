use crate::graphics;


pub struct Input {
    pub cursor: Pointer,
}

impl Input {
    pub fn new() -> Self {
        Self {
            cursor: None,
        }
    }
}


pub type Pointer = Option<graphics::Pnt2>;
