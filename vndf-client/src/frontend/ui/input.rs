use crate::graphics;

#[derive(Debug)]
pub struct Input {
    pub cursor: Cursor,
    pub click: bool,

    pub actions: Vec<Action>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            cursor: None,
            click: false,

            actions: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.click = false;
    }
}

pub type Cursor = Option<graphics::Pnt2>;

#[derive(Debug)]
pub enum Action {
    AddCommand,
}
