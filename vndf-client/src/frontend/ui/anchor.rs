use crate::{
    frontend::{
        drawers::Frame,
        ui::elements::Widget,
    },
    graphics,
};


pub struct Anchor {
    pub horizontal: Horizontal,
    pub vertical:   Vertical,
}

impl Anchor {
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

    pub fn origin(self, frame: &Frame) -> Origin {
        let size = frame.screen.logical_size();

        let x = match self.horizontal {
            Horizontal::Left  => 0.0,
            Horizontal::Right => size.width,
        };
        let y = match self.vertical {
            Vertical::Top    => 0.0,
            Vertical::Bottom => size.height,
        };

        let pos = graphics::Pnt2::new(x, y);

        Origin {
            anchor: self,
            pos,
        }
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


pub struct Origin {
    anchor: Anchor,
    pos:    graphics::Pnt2,
}

impl Origin {
    pub fn position(self,
        element: &dyn Widget,
        margin:  graphics::Scalar,
    )
        -> graphics::Pnt2
    {
        self.pos + element.offset(self.anchor, margin)
    }
}
