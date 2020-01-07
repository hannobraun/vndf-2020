use crate::{
    game::{
        components::{
            Body,
            Engine,
            Explosion,
            Missile,
        },
        events,
    },
    world,
};


pub fn update_missiles(world: world::Query, events: &mut events::Push) {
    let query = &mut world.query::<(&Missile, &Body, &Engine)>();
    for (id, (missile, body, engine)) in query {
        if let Some(explosion) = missile.update(body, engine) {
            events.explode_missile(id, explosion);
        }
    }
}

pub fn update_explosions(
    world:  world::Query,
    dt:     f32,
    events: &mut events::Push,
) {
    for (id, (explosion,)) in &mut world.query::<(&mut Explosion,)>() {
        if explosion.update(dt) {
            events.remove_explosion(id);
        }
    }
}
