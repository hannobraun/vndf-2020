use std::net::SocketAddr;

use crate::{
    cgs::Store,
    events,
    game::{
        PlayerId,
        entities,
        features::{
            crafts::components::Craft,
            missiles::events::MissileLaunch,
            physics::components::Body,
            players::{
                events::PlayerItemCreated,
                items::Player,
            },
            ships::components::Ship,
        },
        indices::Indices,
    },
    input,
    world,
};


pub fn connect_player(
    world:               &mut world::Spawn,
    players:             &mut Store<Player>,
    player_item_created: &mut events::Sink<PlayerItemCreated>,
    indices:             &mut Indices,
    id:                  PlayerId,
    addr:                SocketAddr,
    color:               [f32; 3],
) {
    let key = players.insert(Player::new(id, addr));
    indices.players_by_address.insert(addr, key);

    world.spawn(entities::ship(id, color));
    player_item_created.push(PlayerItemCreated { id, addr });
}

pub fn disconnect_player(
    players: &mut Store<Player>,
    indices: &mut Indices,
    address: SocketAddr,
) {
    // It's possible that we're getting multiple disconnect events per player,
    // so the ship could have been removed already.
    if let Some(handle) = indices.players_by_address.remove(&address) {
        players.remove(handle);

        // In principle, an event needs to be emitted to mark the removal of the
        // item. Eventually, this should happen automatically, but in the
        // meantime, systems need to do this manually.
        //
        // Neither is happening here. As of this writing, no item removal
        // infrastructure exists yet, and since removing players isn't required
        // for the correct functioning of the game, I've opted to leave this be
        // for now.
    }
}

pub fn handle_input(
    world:          world::Query,
    players:        &Store<Player>,
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
    let player: Player = *players.get(player)
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
