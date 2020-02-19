use std::{
    collections::{
        HashMap,
        HashSet,
    },
    net::SocketAddr,
};

use rinnsal::{
    EventBuf,
    EventSink,
};
use serde::{
    Deserialize,
    Serialize,
};
use toadster::{
    handle::{
        self,
        Untyped,
    },
    store,
};

use crate::game::{
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
    players_by_address: HashMap<SocketAddr, handle::Strong<Player>>,

    pub players: store::Strong<Player>,

    pub input_handled:       EventBuf<InputHandled>,
    pub player_connected:    EventBuf<PlayerConnected>,
    pub player_created:      EventBuf<PlayerCreated>,
    pub player_disconnected: EventBuf<PlayerDisconnected>,
    pub player_input:        EventBuf<PlayerInput>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            next_id:            PlayerId::first(),
            players_by_address: HashMap::new(),

            players: store::Strong::new(),

            input_handled:       EventBuf::new(),
            player_connected:    EventBuf::new(),
            player_created:      EventBuf::new(),
            player_disconnected: EventBuf::new(),
            player_input:        EventBuf::new(),
        }
    }

    pub fn on_player_connected(&mut self,
        event:      &PlayerConnected,
        bodies:     &mut store::Strong<Body>,
        crafts:     &mut store::Strong<Craft>,
        directions: &mut store::Strong<Direction>,
        fuels:      &mut store::Strong<Fuel>,
        healths:    &mut store::Strong<Health>,
        positions:  &mut store::Strong<Position>,
        ships:      &mut store::Strong<Ship>,
        velocities: &mut store::Strong<Velocity>,
        entities:   &mut HashSet<handle::Strong<Untyped>>,
    ) {
        connect_player(
            self.next_id.increment(),
            event.addr,
            event.color,
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
            entities,
        );
    }

    pub fn on_player_disconnected(&mut self, event: &PlayerDisconnected) {
        disconnect_player(
            &mut self.players_by_address,
            event.addr,
        );
    }

    pub fn on_player_input(&mut self,
        event:          &PlayerInput,
        bodies:         &store::Strong<Body>,
        crafts:         &mut store::Strong<Craft>,
        ships:          &mut store::Strong<Ship>,
        missile_launch: &mut EventSink<MissileLaunch>,
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
