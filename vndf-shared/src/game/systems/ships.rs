use std::net::SocketAddr;

use crate::{
    game::{
        components::{
            Body,
            Engine,
            Ship,
        },
        events,
    },
    input,
    world,
};


pub fn handle_input(
    world:  world::Query,
    events: &mut events::Push,
    player: SocketAddr,
    input:  input::Event,
) {
    let query = &mut world.query::<(&mut Ship, &Body, &mut Engine)>();

    for (_, (ship, body, engine)) in query {
        if ship.player != player {
            continue;
        }

        match input {
            input::Event::Rotate(rotation) => {
                ship.rotation = rotation;
            }
            input::Event::Thrust(thrust) => {
                engine.enabled = thrust;
            }
            input::Event::LaunchMissile => {
                if let Some(missile) = ship.launch_missile(body) {
                    events.launch_missile(missile);
                }
            }
        }
    }
}

pub fn update_ships(world: world::Query) {
    for (_, (ship, body)) in &mut world.query::<(&mut Ship, &mut Body)>() {
        ship.update(body);
    }
}
