use std::net::SocketAddr;

use crate::{
    game::{
        components::{
            Body,
            Engine,
            Ship,
        },
        events::Events,
    },
    input::Rotation,
    world,
};


pub fn handle_rotate(
    world:    &mut world::Query,
    player:   SocketAddr,
    rotation: Rotation,
) {
    for (_, (ship,)) in &mut world.query::<(&mut Ship,)>() {
        if ship.player == player {
            ship.rotation = rotation;
        }
    }
}

pub fn handle_thrust(
    world:  &mut world::Query,
    player: SocketAddr,
    thrust: bool,
) {
    for (_, (ship, engine)) in &mut world.query::<(&Ship, &mut Engine)>() {
        if ship.player == player {
            engine.enabled = thrust;
        }
    }
}

pub fn handle_launch(
    world:  &mut world::Query,
    player: SocketAddr,
    events: &mut Events,
) {
    for (_, (ship, body)) in &mut world.query::<(&mut Ship, &Body)>() {
        if ship.player == player {
            if let Some(missile) = ship.launch_missile(body) {
                events.push().launch_missile(missile);
            }
        }
    }
}
