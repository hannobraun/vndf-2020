use serde::{
    Deserialize,
    Serialize,
};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Planet {
    pub size: f32,
}

impl Planet {
    pub fn to_weak(&self) -> Self {
        self.clone()
    }
}
