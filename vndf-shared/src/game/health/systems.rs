use std::collections::HashSet;
use rinnsal::EventSink;
use toadster::{
    handle::{
        self,
        Untyped,
    },
    store,
};

use super::{
    components::Health,
    events::Death,
};


pub fn check_health(
    healths: &store::Strong<Health>,
    death:   &mut EventSink<Death>,
    index:   &mut HashSet<handle::Strong<Untyped>>,
) {
    for (handle, health) in healths.iter().strong() {
        if health.is_dead() {
            let parent = health
                .parent_ref()
                .unwrap()
                .clone()
                .into_weak_untyped();
            index.remove(&parent);

            death.push(Death { handle });
        }
    }
}
