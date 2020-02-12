use std::{
    collections::HashMap,
    net::SocketAddr,
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    cgs::{
        Handle,
        Store,
    },
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
        ships::Ship,
    },
    events,
};

use super::{
    InputHandled,
    Player,
    PlayerConnected,
    PlayerCreated,
    PlayerDisconnected,
    PlayerInput,
    connect_player,
    disconnect_player,
    handle_input,
};


pub struct Feature {
    next_id:            PlayerId,
    players_by_address: HashMap<SocketAddr, Handle<Player>>,

    pub players: Store<Player>,

    pub input_handled:       events::Buf<InputHandled>,
    pub player_connected:    events::Buf<PlayerConnected>,
    pub player_created:      events::Buf<PlayerCreated>,
    pub player_disconnected: events::Buf<PlayerDisconnected>,
    pub player_input:        events::Buf<PlayerInput>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            next_id:            PlayerId::first(),
            players_by_address: HashMap::new(),

            players: Store::new(),

            input_handled:       events::Buf::new(),
            player_connected:    events::Buf::new(),
            player_created:      events::Buf::new(),
            player_disconnected: events::Buf::new(),
            player_input:        events::Buf::new(),
        }
    }

    pub fn on_player_connected(&mut self,
        event:      &PlayerConnected,
        bodies:     &mut Store<Body>,
        crafts:     &mut Store<Craft>,
        directions: &mut Store<Direction>,
        fuels:      &mut Store<Fuel>,
        healths:    &mut Store<Health>,
        positions:  &mut Store<Position>,
        ships:      &mut Store<Ship>,
        velocities: &mut Store<Velocity>,
    ) {
        connect_player(
            bodies,
            crafts,
            directions,
            fuels,
            healths,
            &mut self.players,
            positions,
            ships,
            velocities,
            &mut self.player_created.sink(),
            &mut self.players_by_address,
            self.next_id.increment(),
            event.addr,
            event.color,
        );
    }

    pub fn on_player_disconnected(&mut self, event: &PlayerDisconnected) {
        disconnect_player(
            &mut self.players,
            &mut self.players_by_address,
            event.addr,
        );
    }

    pub fn on_player_input(&mut self,
        event:          &PlayerInput,
        bodies:         &Store<Body>,
        crafts:         &mut Store<Craft>,
        ships:          &mut Store<Ship>,
        missile_launch: &mut events::Sink<MissileLaunch>,
    ) {
        handle_input(
            event.addr,
            event.action,
            bodies,
            crafts,
            &self.players,
            ships,
            missile_launch,
            &mut self.input_handled.sink(),
            &mut self.players_by_address,
        );
    }
}


#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct PlayerId(u64);

impl PlayerId {
    pub fn first() -> Self {
        Self(0)
    }

    pub fn increment(&mut self) -> Self {
        let current = self.0;
        self.0 += 1;
        Self(current)
    }
}
