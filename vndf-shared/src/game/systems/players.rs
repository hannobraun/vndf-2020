use std::net::SocketAddr;

use crate::{
    game::{
        PlayerId,
        components::{
            Body,
            Craft,
            Ship,
        },
        entities,
        events,
        indices::Indices,
    },
    input,
    world,
};


pub fn connect_player(
    world:   &mut world::Spawn,
    indices: &mut Indices,
    id:      PlayerId,
    address: SocketAddr,
) {
    let entity = world.spawn(entities::player(id));
    indices.players.insert(address, entity);

    world.spawn(entities::ship(id, address));
}

pub fn disconnect_player(
    world:   &mut world::Spawn,
    indices: &mut Indices,
    player:  SocketAddr,
) {
    // It's possible that we're getting multiple disconnect events per player,
    // so the ship could have been removed already.
    if let Some(entity) = indices.players.remove(&player) {
        world.despawn(entity)
            .expect("Tried to remove ship that doesn't exist")
    }
}

pub fn handle_input(
    world:  world::Query,
    events: &mut events::Push,
    player: SocketAddr,
    input:  input::Event,
) {
    let query = &mut world.query::<(&mut Ship, &Body, &mut Craft)>();

    for (_, (ship, body, craft)) in query {
        if ship.player != player {
            continue;
        }

        match input {
            input::Event::Rotate(rotation) => {
                ship.rotation = rotation;
            }
            input::Event::Thrust(thrust) => {
                craft.engine_on = thrust;
            }
            input::Event::LaunchMissile { target } => {
                let missile = ship.launch_missile(craft.owner, body, target);
                if let Some(missile) = missile {
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
