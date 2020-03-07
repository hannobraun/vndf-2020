use std::{
    collections::{
        HashMap,
        HashSet,
    },
    net::SocketAddr,
};

use log::warn;
use rinnsal::EventSink;
use toadster::{
    handle::{
        self,
        Untyped,
    },
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
    id:             PlayerId,
    addr:           SocketAddr,
    color:          [f32; 3],
    bodies:         &mut store::Strong<Body>,
    crafts:         &mut store::Strong<Craft>,
    fuels:          &mut store::Strong<Fuel>,
    healths:        &mut store::Strong<Health>,
    players:        &mut store::Strong<Player>,
    positions:      &mut store::Strong<Position>,
    ships:          &mut store::Strong<Ship>,
    velocities:     &mut store::Strong<Velocity>,
    player_created: &mut EventSink<PlayerCreated>,
    index:          &mut HashMap<SocketAddr, handle::Strong<Player>>,
    entities:       &mut HashSet<handle::Strong<Untyped>>,
) {
    let handle = players.insert(Player::new(id, addr));
    index.insert(addr, handle);

    ShipEntity { owner: id, color }.create(
        bodies,
        crafts,
        fuels,
        healths,
        positions,
        ships,
        velocities,
        entities,
    );
    player_created.push(PlayerCreated { id, addr });
}

pub fn disconnect_player(
    index:   &mut HashMap<SocketAddr, handle::Strong<Player>>,
    address: SocketAddr,
) {
    index.remove(&address);
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
    index:          &mut HashMap<SocketAddr, handle::Strong<Player>>,
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
