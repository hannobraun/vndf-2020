#[derive(Clone, Copy)]
pub struct Explosion {
    pub time_total: f32,
    pub time_left:  f32,
}

impl Explosion {
    pub fn new() -> Self {
        let time_total = 3.0;

        Self {
            time_total,
            time_left: time_total,
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        if self.time_left > 0.0 {
            self.time_left -= dt;
            false
        }
        else {
            true
        }
    }
}
