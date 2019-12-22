use ggez::mint::Point2;


pub struct State {
    pub position: Point2<f32>,
}

impl State {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0].into(),
        }
    }
}
