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
            base::Update,
            players::{
                PlayerConnected,
                PlayerDisconnected,
                PlayerInput,
            },
        },
        net::{
            self,
            data::Data,
            msg,
        },
    },
};


pub struct Server {
    network:     Network,
    events:      Vec<Event>,
    state:       game::State,
    last_update: Instant,
    data:        Data,
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
            data:        Data::new(),
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
                Event::Message(addr, msg::FromClient::Input(event)) => {
                    debug!("Input from {}: {:?}", addr, event);
                    self.state.player_input().push(PlayerInput { addr, event });
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

        for player in self.state.player_created().ready() {
            self.network.send(
                player.addr,
                msg::FromServer::Welcome(player.id),
            );
        }

        for event in self.state.component_removed().ready() {
            self.data.remove(event.handle);

            for &client in &clients {
                self.network.send(
                    client,
                    msg::FromServer::RemoveComponent(event.handle),
                );
            }
        }

        for (handle, component) in self.state.updates() {
            let data_changed = self.data.update(handle, component);

            if data_changed {
                for &client in &clients {
                    self.network.send(
                        client,
                        msg::FromServer::UpdateComponent(handle, component),
                    );
                }
            }
        }

        for &client in &clients {
            self.network.send(
                client,
                msg::FromServer::Diagnostics(self.state.diagnostics()),
            );
        }
    }
}
