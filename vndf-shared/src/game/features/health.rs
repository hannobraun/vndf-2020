use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    events,
    game::in_event::InEvent,
    world,
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Health {
    pub value: f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Health {
            value
        }
    }

    pub fn is_dead(&self) -> bool {
        self.value <= 0.0
    }
}


pub fn check_health(
    world:  world::Query,
    events: &mut events::Push<InEvent>,
) {
    for (entity, (health,)) in &mut world.query::<(&Health,)>() {
        if health.is_dead() {
            events.dead_entity(entity);
        }
    }
}
