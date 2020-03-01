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
    pub death: EventBuf<Death>,
    pub index: HashSet<handle::Strong<Untyped>>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            death: EventBuf::new(),
            index: HashSet::new(),
        }
    }

    pub fn on_update(&mut self,
        healths: &store::Strong<Health>,
    ) {
        check_health(
            healths,
            &mut self.death.sink(),
            &mut self.index,
        );
    }
}
