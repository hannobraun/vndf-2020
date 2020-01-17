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
            out_event::OutEvent,
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
                Event::Message(id, msg::FromClient::Hello) => {
                    info!("Connected: {}", id);
                    self.state.push().connect_player(id);
                }
                Event::Message(id, msg::FromClient::Input(input)) => {
                    debug!("Input from {}: {:?}", id, input);
                    self.state.push().player_input(id, input);
                }
                Event::Error(id, _) => {
                    info!("Disconnected: {}", id);
                    self.state.push().disconnect_player(id);
                }
            }
        }

        let now        = Instant::now();
        let frame_time = Duration::from_millis((FRAME_TIME * 1000.0) as u64);

        while now.duration_since(self.last_update) > frame_time {
            self.state.push().update(FRAME_TIME);
            self.state.dispatch();
            self.last_update += frame_time;
        }

        let clients = self.state.players();

        for event in self.state.out_events() {
            match event {
                OutEvent::Despawn { entity } => {
                    for &address in &clients {
                        self.network.send(
                            address,
                            msg::FromServer::RemoveEntity(
                                Id::from_hecs_entity(&entity)
                            ),
                        );
                    }
                }
                OutEvent::CreatePlayer { addr, .. } => {
                    self.network.send(addr, msg::FromServer::Welcome(addr));
                }
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
