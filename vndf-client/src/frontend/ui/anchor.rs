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
}


pub enum Horizontal {
    Left,
    Right,
}

pub enum Vertical {
    Top,
    Bottom,
}
