use std::collections::HashSet;

use rinnsal::EventBuf;
use toadster::{
    handle::{
        self,
        Untyped,
    },
    store,
};

use super::{
    Death,
    Health,
    check_health,
};


pub struct Feature {
    pub healths: store::Strong<Health>,
    pub death:   EventBuf<Death>,
    pub index:   HashSet<handle::Strong<Untyped>>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            healths: store::Strong::new(),
            death:   EventBuf::new(),
            index:   HashSet::new(),
        }
    }

    pub fn on_update(&mut self) {
        check_health(
            &self.healths,
            &mut self.death.sink(),
            &mut self.index,
        );
    }
}
