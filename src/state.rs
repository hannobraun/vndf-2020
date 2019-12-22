use crate::math::Pnt2;


pub struct State {
    pub position: Pnt2,
}

impl State {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0].into(),
        }
    }
}
