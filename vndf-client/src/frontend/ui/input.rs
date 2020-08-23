use crate::graphics;


pub struct Input {
    pub cursor: Cursor,
}

impl Input {
    pub fn new() -> Self {
        Self {
            cursor: None,
        }
    }
}


pub type Cursor = Option<graphics::Pnt2>;
