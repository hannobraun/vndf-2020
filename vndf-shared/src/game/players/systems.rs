use std::{
    collections::HashMap,
    net::SocketAddr,
};

use bach::EventSink;
use log::warn;
use toadster::{
    StrongHandle,
    store,
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
    bodies:         &mut store::Strong<Body>,
    crafts:         &mut store::Strong<Craft>,
    directions:     &mut store::Strong<Direction>,
    fuels:          &mut store::Strong<Fuel>,
    healths:        &mut store::Strong<Health>,
    players:        &mut store::Strong<Player>,
    positions:      &mut store::Strong<Position>,
    ships:          &mut store::Strong<Ship>,
    velocities:     &mut store::Strong<Velocity>,
    player_created: &mut EventSink<PlayerCreated>,
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
    players: &mut store::Strong<Player>,
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
    bodies:         &store::Strong<Body>,
    crafts:         &mut store::Strong<Craft>,
    players:        &store::Strong<Player>,
    ships:          &mut store::Strong<Ship>,
    missile_launch: &mut EventSink<MissileLaunch>,
    input_handled:  &mut EventSink<InputHandled>,
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
