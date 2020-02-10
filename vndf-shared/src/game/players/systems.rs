use std::{
    collections::HashMap,
    net::SocketAddr,
};

use log::warn;

use crate::{
    cgs::{
        Handle,
        Store,
    },
    events,
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
    input,
};

use super::{
    InputHandled,
    Player,
    PlayerCreated,
};


pub fn connect_player(
    bodies:         &mut Store<Body>,
    crafts:         &mut Store<Craft>,
    directions:     &mut Store<Direction>,
    fuels:          &mut Store<Fuel>,
    healths:        &mut Store<Health>,
    players:        &mut Store<Player>,
    positions:      &mut Store<Position>,
    ships:          &mut Store<Ship>,
    velocities:     &mut Store<Velocity>,
    player_created: &mut events::Sink<PlayerCreated>,
    index:          &mut HashMap<SocketAddr, Handle>,
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
    players: &mut Store<Player>,
    index:   &mut HashMap<SocketAddr, Handle>,
    address: SocketAddr,
) {
    // It's possible that we're getting multiple disconnect events per player,
    // so the ship could have been removed already.
    if let Some(handle) = index.remove(&address) {
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
    addr:           SocketAddr,
    input:          input::Event,
    bodies:         &Store<Body>,
    crafts:         &mut Store<Craft>,
    players:        &Store<Player>,
    ships:          &mut Store<Ship>,
    missile_launch: &mut events::Sink<MissileLaunch>,
    input_handled:  &mut events::Sink<InputHandled>,
    index:          &mut HashMap<SocketAddr, Handle>,
)
    -> Option<()>
{
    let player = index.get(&addr)
        .or_else(|| {
            warn!("Player not in index: {}", addr);
            None
        })?;
    let player = players.get(*player)
        .or_else(|| {
            warn!("Player component not found: {}", addr);
            None
        })?;

    for ship in ships.values_mut() {
        ship.apply_input(bodies, crafts, missile_launch, player, input);
    }

    input_handled.push(InputHandled { addr, seq: input.seq });

    Some(())
}
