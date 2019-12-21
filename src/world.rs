use ggez::mint::Point2;


pub struct World {
    pub position: Point2<f32>,
}

impl World {
    pub fn new() -> Self {
        World {
            position: [0.0, 0.0].into(),
        }
    }
}
