use serde::{
    Deserialize,
    Serialize,
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Health {
    pub parent: Option<u64>,
    pub value:  f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Health {
            parent: None,
            value
        }
    }

    pub fn is_dead(&self) -> bool {
        self.value <= 0.0
    }
}
