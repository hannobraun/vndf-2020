use crate::{
    frontend::drawers::Frame,
    graphics,
};


pub struct Origin {
    pub horizontal: Horizontal,
    pub vertical:   Vertical,
}

impl Origin {
    pub fn top_left() -> Self {
        Self {
            horizontal: Horizontal::Left,
            vertical:   Vertical::Top,
        }
    }

    pub fn top_right() -> Self {
        Self {
            horizontal: Horizontal::Right,
            vertical:   Vertical::Top,
        }
    }

    pub fn bottom_left() -> Self {
        Self {
            horizontal: Horizontal::Left,
            vertical:   Vertical::Bottom,
        }
    }

    pub fn bottom_right() -> Self {
        Self {
            horizontal: Horizontal::Right,
            vertical:   Vertical::Bottom,
        }
    }

    pub fn point(&self, frame: &Frame) -> graphics::Pnt2 {
        let size = frame.screen.logical_size();

        let x = match self.horizontal {
            Horizontal::Left  => 0.0,
            Horizontal::Right => size.width,
        };
        let y = match self.vertical {
            Vertical::Top    => 0.0,
            Vertical::Bottom => size.height,
        };

        graphics::Pnt2::new(x, y)
    }
}


pub enum Horizontal {
    Left,
    Right,
}

pub enum Vertical {
    Top,
    Bottom,
}
