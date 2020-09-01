use crate::graphics;


pub struct Input {
    pub cursor: Cursor,
    pub click:  bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            cursor: None,
            click:  false,
        }
    }

    pub fn reset(&mut self) {
        self.click = false;
    }
}


pub type Cursor = Option<graphics::Pnt2>;
