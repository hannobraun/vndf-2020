use crate::cgs::Store;

use super::Loot;


pub struct Feature {
    pub loots: Store<Loot>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            loots: Store::new(),
        }
    }
}
