use serde::{
    Deserialize,
    Serialize,
};

use crate::math::Pnt2;


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Planet {
    pub pos:  Pnt2,
    pub size: f32,
}

impl Planet {
    pub fn to_weak(&self) -> Self {
        self.clone()
    }
}
