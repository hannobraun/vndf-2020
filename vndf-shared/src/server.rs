use std::{
    net::SocketAddr,
    time::{
        Duration,
        Instant,
    },
};

use crate::{
    game::{
        self,
        FRAME_TIME,
    },
    net::{
        self,
        Network,
        game::{
            Entity,
            Id,
        },
        msg,
        network,
    },
};


pub struct Server {
    network:     Network,
    events:      Vec<network::Event>,
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
                network::Event::Message(id, msg::FromClient::Hello) => {
                    self.network.send(id, msg::FromServer::Welcome);
                }
                network::Event::Message(_id, msg::FromClient::Input(input)) => {
                    self.state.handle_input(input);
                }
                _ => (),
            }
        }

        let now        = Instant::now();
        let frame_time = Duration::from_millis((FRAME_TIME * 1000.0) as u64);

        while now.duration_since(self.last_update) > frame_time {
            self.state.update(FRAME_TIME);
            self.last_update += frame_time;
        }

        let spawned_id: Vec<_> = self.state.spawned().collect();
        let mut spawned = Vec::new();
        for entity in spawned_id {
            spawned.push(Entity::from_world(entity, &self.state.world));
        }

        let despawned: Vec<_> = self.state.despawned().collect();

        let mut updated = Vec::new();
        for (entity, _) in self.state.world.iter() {
            updated.push(Entity::from_world(entity, &self.state.world));
        }

        let clients: Vec<SocketAddr> = self.network.clients().collect();
        for client in clients {
            for entity in &spawned {
                self.network.send(
                    client,
                    msg::FromServer::AddEntity(entity.clone()),
                );
            }
            for entity in &despawned {
                self.network.send(
                    client,
                    msg::FromServer::RemoveEntity(Id::from_hecs_entity(entity)),
                );
            }
            for entity in &updated {
                self.network.send(
                    client,
                    msg::FromServer::UpdateEntity(entity.clone()),
                );
            }
        }
    }
}
