use std::net::SocketAddr;

use crate::{
    events,
    game::{
        PlayerId,
        components::Ship,
        entities,
        features::{
            crafts::components::Craft,
            missiles::events::MissileLaunch,
            physics::components::Body,
            players::{
                components::Player,
                events::PlayerEntityCreated,
            },
        },
        indices::Indices,
    },
    input,
    world,
};


pub fn connect_player(
    world:                 &mut world::Spawn,
    player_entity_created: &mut events::Sink<PlayerEntityCreated>,
    indices:               &mut Indices,
    id:                    PlayerId,
    addr:                  SocketAddr,
    color:                 [f32; 3],
) {
    let entity = world.spawn(entities::player(id, addr));
    indices.players_by_address.insert(addr, entity);

    world.spawn(entities::ship(id, color));
    player_entity_created.push(PlayerEntityCreated { id, addr });
}

pub fn disconnect_player(
    world:   &mut world::Spawn,
    indices: &mut Indices,
    address: SocketAddr,
) {
    // It's possible that we're getting multiple disconnect events per player,
    // so the ship could have been removed already.
    if let Some(entity) = indices.players_by_address.remove(&address) {
        world.despawn(entity)
            .expect("Tried to remove ship that doesn't exist")
    }
}

pub fn handle_input(
    world:          world::Query,
    missile_launch: &mut events::Sink<MissileLaunch>,
    indices:        &mut Indices,
    address:        SocketAddr,
    input:          input::Event,
) {
    let player = match indices.players_by_address.get(&address) {
        Some(player) =>
            *player,
        // Ignore input from unknown player.
        None =>
            return,
    };
    let player: Player = *world.get(player)
        .expect("Couldn't find player despite getting id from index");

    let query = &mut world.query::<(&mut Ship, &Body, &mut Craft)>();
    for (_, (ship, body, craft)) in query {
        if craft.owner != player.id {
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
                    missile_launch.push(MissileLaunch { missile });
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
