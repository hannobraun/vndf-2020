use crate::{
    game::components::{
        Body,
        Engine,
        Ship,
    },
    input::Rotation,
    world,
};


pub fn handle_rotate(world: &mut world::Query, rotation: Rotation) {
    for (_, (ship,)) in &mut world.query::<(&mut Ship,)>() {
        ship.rotation = rotation;
    }
}

pub fn handle_thrust(world: &mut world::Query, thrust: bool) {
    for (_, (_, engine)) in &mut world.query::<(&Ship, &mut Engine)>() {
        engine.enabled = thrust;
    }
}

pub fn handle_launch(world: &mut world::Query) {
    let mut missiles = Vec::new();
    {
        for (_, (ship, body)) in &mut world.query::<(&mut Ship, &Body)>() {
            if let Some(missile) = ship.launch_missile(body) {
                missiles.push(missile);
            }
        }
    }

    for missile in missiles {
        world.spawn(missile);
    }
}