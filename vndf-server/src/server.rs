use std::{
    net::SocketAddr,
    time::{
        Duration,
        Instant,
    },
};

use log::{
    debug,
    info,
};

use crate::{
    net::{
        Event,
        Network,
    },
    shared::{
        game::{
            self,
            FRAME_TIME,
            features::{
                basics::Update,
                players::{
                    PlayerConnected,
                    PlayerDisconnected,
                },
            },
        },
        net::{
            self,
            game::{
                Entity,
                Id,
            },
            msg,
        },
    },
};


pub struct Server {
    network:     Network,
    events:      Vec<Event>,
    state:       game::State,
    last_update: Instant,
}

impl Server {
    pub fn start_default() -> net::Result<Self> {
        Ok(Self::new(Network::start_default()?))
    }

    pub fn start_local() -> net::Result<Self> {
        Ok(Self::new(Network::start_local()?))
    }

    fn new(network: Network) -> Self {
        Self {
            network,
            events:      Vec::new(),
            state:       game::State::new(),
            last_update: Instant::now(),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.network.addr()
    }

    pub fn update(&mut self) {
        self.events.extend(self.network.events());

        for event in self.events.drain(..) {
            match event {
                Event::Message(_, msg::FromClient::Ping) => {
                    // This message is just for testing purposes. Nothing to do
                    // here.
                }
                Event::Message(addr, msg::FromClient::Hello { color }) => {
                    info!("Connected: {}", addr);
                    // Yes, it's a bad idea to just trust the client to provide
                    // a color that is not the same as the background color.
                    // It's good enough for now though.
                    self.state.player_connected()
                        .push(PlayerConnected { addr, color });
                }
                Event::Message(id, msg::FromClient::Input(input)) => {
                    debug!("Input from {}: {:?}", id, input);
                    self.state.push().player_input(id, input);
                }
                Event::Error(addr, _) => {
                    info!("Disconnected: {}", addr);
                    self.state.player_disconnected()
                        .push(PlayerDisconnected { addr });
                }
            }
        }

        let now        = Instant::now();
        let frame_time = Duration::from_millis((FRAME_TIME * 1000.0) as u64);

        while now.duration_since(self.last_update) > frame_time {
            self.state.update().push(Update { dt: FRAME_TIME });
            self.state.dispatch();
            self.last_update += frame_time;
        }

        let clients = self.state.players();

        for new_player in self.state.new_player().ready() {
            self.network.send(
                new_player.addr,
                msg::FromServer::Welcome(new_player.id),
            );
        }
        for entity_removed in self.state.entity_removed().ready() {
            for &address in &clients {
                self.network.send(
                    address,
                    msg::FromServer::RemoveEntity(
                        Id::from_hecs_entity(&entity_removed.entity)
                    ),
                );
            }
        }

        let mut updated = Vec::new();
        for (entity, _) in self.state.world().inner().iter() {
            updated.push(
                Entity::from_world(entity, self.state.world().inner()),
            );
        }

        for client in clients {
            for entity in &updated {
                self.network.send(
                    client,
                    msg::FromServer::UpdateEntity(entity.clone()),
                );
            }
        }
    }
}
