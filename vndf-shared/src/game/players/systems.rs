use std::{
    collections::HashMap,
    net::SocketAddr,
};

use log::warn;
use toadster::{
    StrongHandle,
    StrongStore,
};

use crate::{
    game::{
        crafts::{
            Craft,
            Fuel,
        },
        health::Health,
        missiles::MissileLaunch,
        physics::{
            Body,
            Direction,
            Position,
            Velocity,
        },
        players::PlayerId,
        ships::{
            Ship,
            ShipEntity,
        },
    },
    input::Action,
};

use super::{
    InputHandled,
    Player,
    PlayerCreated,
};


pub fn connect_player(
    bodies:         &mut StrongStore<Body>,
    crafts:         &mut StrongStore<Craft>,
    directions:     &mut StrongStore<Direction>,
    fuels:          &mut StrongStore<Fuel>,
    healths:        &mut StrongStore<Health>,
    players:        &mut StrongStore<Player>,
    positions:      &mut StrongStore<Position>,
    ships:          &mut StrongStore<Ship>,
    velocities:     &mut StrongStore<Velocity>,
    player_created: &mut bach::Sink<PlayerCreated>,
    index:          &mut HashMap<SocketAddr, StrongHandle<Player>>,
    id:             PlayerId,
    addr:           SocketAddr,
    color:          [f32; 3],
) {
    let handle = players.insert(Player::new(id, addr));
    index.insert(addr, handle);

    ShipEntity { owner: id, color }.create(
        bodies,
        crafts,
        directions,
        fuels,
        healths,
        positions,
        ships,
        velocities,
    );
    player_created.push(PlayerCreated { id, addr });
}

pub fn disconnect_player(
    players: &mut StrongStore<Player>,
    index:   &mut HashMap<SocketAddr, StrongHandle<Player>>,
    address: SocketAddr,
) {
    // It's possible that we're getting multiple disconnect events per player,
    // so the ship could have been removed already.
    if let Some(handle) = index.remove(&address) {
        players.remove(handle);
    }
}

pub fn handle_input(
    addr:           SocketAddr,
    action:         Action,
    bodies:         &StrongStore<Body>,
    crafts:         &mut StrongStore<Craft>,
    players:        &StrongStore<Player>,
    ships:          &mut StrongStore<Ship>,
    missile_launch: &mut bach::Sink<MissileLaunch>,
    input_handled:  &mut bach::Sink<InputHandled>,
    index:          &mut HashMap<SocketAddr, StrongHandle<Player>>,
)
    -> Option<()>
{
    let player = index.get(&addr)
        .or_else(|| {
            warn!("Player not in index: {}", addr);
            None
        })?;
    let player = players.get(player)
        .or_else(|| {
            warn!("Player component not found: {}", addr);
            None
        })?;

    for ship in ships.values_mut() {
        ship.apply_input(bodies, crafts, missile_launch, player, action);
    }

    input_handled.push(InputHandled { addr, seq: action.seq });

    Some(())
}
