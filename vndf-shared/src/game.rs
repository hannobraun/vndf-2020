pub mod base;
pub mod crafts;
pub mod explosions;
pub mod health;
pub mod missiles;
pub mod physics;
pub mod players;
pub mod ships;


use std::{
    collections::HashMap,
    net::SocketAddr,
};

use crate::{
    cgs::{
        Handle,
        Store,
    },
    events,
    world::World,
};

use self::{
    base::{
        Component,
        ComponentHandle,
        EntityRemoved,
        ItemRemoved,
        Update,
    },
    crafts::Craft,
    explosions::{
        Explosion,
        ExplosionFaded,
        ExplosionImminent,
    },
    health::{
        Death,
        Health,
    },
    missiles::{
        Missile,
        MissileLaunch,
    },
    physics::Body,
    players::{
        Player,
        PlayerConnected,
        PlayerCreated,
        PlayerDisconnected,
        PlayerId,
        PlayerInput,
    },
    ships::Ship,
};


pub const WORLD_SIZE: f32 = 1000.0;

pub const TARGET_FPS: u32 = 60;
pub const FRAME_TIME: f32 = 1.0 / TARGET_FPS as f32;


pub struct State {
    world:   World,
    next_id: PlayerId,

    players_by_address: HashMap<SocketAddr, Handle>,

    bodies:     Store<Body>,
    crafts:     Store<Craft>,
    explosions: Store<Explosion>,
    players:    Store<Player>,
    missiles:   Store<Missile>,
    ships:      Store<Ship>,

    death:               events::Buf<Death>,
    entity_removed:      events::Buf<EntityRemoved>,
    explosion_faded:     events::Buf<ExplosionFaded>,
    explosion_imminent:  events::Buf<ExplosionImminent>,
    item_removed:        events::Buf<ItemRemoved>,
    missile_launch:      events::Buf<MissileLaunch>,
    player_connected:    events::Buf<PlayerConnected>,
    player_created:      events::Buf<PlayerCreated>,
    player_disconnected: events::Buf<PlayerDisconnected>,
    player_input:        events::Buf<PlayerInput>,
    update:              events::Buf<Update>,
}

impl State {
    pub fn new() -> Self {
        Self {
            world:   World::new(),
            next_id: PlayerId::first(),

            players_by_address: HashMap::new(),

            bodies:     Store::new(),
            crafts:     Store::new(),
            explosions: Store::new(),
            missiles:   Store::new(),
            players:    Store::new(),
            ships:      Store::new(),

            death:               events::Buf::new(),
            entity_removed:      events::Buf::new(),
            explosion_faded:     events::Buf::new(),
            explosion_imminent:  events::Buf::new(),
            item_removed:        events::Buf::new(),
            missile_launch:      events::Buf::new(),
            player_connected:    events::Buf::new(),
            player_created:      events::Buf::new(),
            player_disconnected: events::Buf::new(),
            player_input:        events::Buf::new(),
            update:              events::Buf::new(),
        }
    }

    pub fn player_connected(&mut self) -> events::Sink<PlayerConnected> {
        self.player_connected.sink()
    }

    pub fn player_disconnected(&mut self) -> events::Sink<PlayerDisconnected> {
        self.player_disconnected.sink()
    }

    pub fn player_input(&mut self) -> events::Sink<PlayerInput> {
        self.player_input.sink()
    }

    pub fn update(&mut self) -> events::Sink<Update> {
        self.update.sink()
    }

    pub fn dispatch(&mut self) {
        // Make sure all healths are linked to their parent. This should happen
        // when they're created, but can't, due to the layers of hacks that is
        // the ECS. It will be possible to rectify this, once we're transitioned
        // the the CGS.
        {
            let world = self.world.query();
            for (handle, (health,)) in &mut world.query::<(&mut Health,)>() {
                health.parent = Some(handle.to_bits());
            }
        }

        // Let's garbage-collect all items that need to be removed. This should
        // be an automatic process, probably controlled by reference-counting of
        // handles, but this will have to do for now.
        let mut remove = Vec::new();
        for (handle, missile) in &self.missiles {
            let entity = hecs::Entity::from_bits(missile.entity);
            if !self.world.query().contains(entity) {
                remove.push(handle);
                let handle = ComponentHandle::Missile(handle);
                self.item_removed.sink().push(ItemRemoved { handle });
            }
        }
        for handle in remove {
            self.missiles.remove(handle);
        }
        let mut remove = Vec::new();
        for (handle, ship) in &self.ships {
            let entity = hecs::Entity::from_bits(ship.entity);
            if !self.world.query().contains(entity) {
                remove.push(handle);
                let handle = ComponentHandle::Ship(handle);
                self.item_removed.sink().push(ItemRemoved { handle });
            }
        }
        for handle in remove {
            self.ships.remove(handle);
        }

        let mut despawned = Vec::new();

        for Update { dt } in self.update.source().ready() {
            ships::update_ships(
                &mut self.bodies,
                &self.crafts,
                &mut self.ships,
            );
            crafts::update_crafts(
                &mut self.bodies,
                &mut self.crafts,
                dt,
            );
            crafts::update_bodies(
                &mut self.bodies,
                WORLD_SIZE,
                dt,
            );
            missiles::update_targets(
                &mut self.bodies,
                &self.crafts,
                &mut self.missiles,
            );
            missiles::update_guidances(
                &mut self.bodies,
                &self.crafts,
                &mut self.missiles,
            );
            missiles::explode_missiles(
                &self.bodies,
                &self.crafts,
                &self.missiles,
                &mut self.world.query(),
            );
            explosions::update_explosions(
                &mut self.explosions,
                dt,
                &mut self.explosion_faded.sink(),
            );
            health::check_health(
                self.world.query(),
                &mut self.death.sink(),
            );
        }
        for event in self.player_connected.source().ready() {
            let PlayerConnected { addr, color } = event;

            let id = self.next_id.increment();

            players::connect_player(
                &mut self.world.spawn(&mut despawned),
                &mut self.bodies,
                &mut self.crafts,
                &mut self.players,
                &mut self.ships,
                &mut self.player_created.sink(),
                &mut self.players_by_address,
                id,
                addr,
                color,
            );
        }
        for event in self.player_disconnected.source().ready() {
            let PlayerDisconnected { addr } = event;

            players::disconnect_player(
                &mut self.players,
                &mut self.players_by_address,
                addr,
            );
        }
        for PlayerInput { addr, event } in self.player_input.source().ready() {
            players::handle_input(
                &self.bodies,
                &mut self.crafts,
                &self.players,
                &mut self.ships,
                &mut self.missile_launch.sink(),
                &mut self.players_by_address,
                addr,
                event,
            );
        }
        for MissileLaunch { missile } in self.missile_launch.source().ready() {
            missiles::launch_missile(
                &mut self.world.spawn(&mut despawned),
                &mut self.bodies,
                &mut self.crafts,
                &mut self.missiles,
                missile,
            );
        }
        for Death { handle } in self.death.source().ready() {
            let explosion = explosions::explode_entity(
                &mut self.world.query(),
                &self.bodies,
                &self.missiles,
                &self.ships,
                handle,
            );
            health::remove_entity(
                &mut self.world.spawn(&mut despawned),
                handle,
            );
            if let Some(explosion) = explosion {
                explosions::create_explosion(
                    &mut self.bodies,
                    &mut self.explosions,
                    &mut self.explosion_imminent.sink(),
                    explosion,
                );
            }
        }
        for event in self.explosion_imminent.source().ready() {
            let ExplosionImminent { handle } = event;

            explosions::damage_nearby(
                &mut self.world.query(),
                &self.bodies,
                &self.explosions,
                handle,
            );
        }
        for event in self.explosion_faded.source().ready() {
            let ExplosionFaded { handle } = event;

            explosions::remove_explosion(
                &mut self.explosions,
                handle,
            );
        }

        for handle in despawned {
            self.entity_removed.sink().push(EntityRemoved { handle });
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn players(&self) -> Vec<SocketAddr> {
        self.players
            .iter()
            .map(|(_, player)| player.addr)
            .collect()
    }

    pub fn updates(&mut self)
        -> impl Iterator<Item=(Handle, Component)> + '_
    {
        let bodies = self.bodies.iter()
            .map(|(handle, &c)| (handle, Component::Body(c)));
        let crafts = self.crafts.iter()
            .map(|(handle, &c)| (handle, Component::Craft(c)));
        let explosions = self.explosions.iter()
            .map(|(handle, &c)| (handle, Component::Explosion(c)));
        let missiles = self.missiles.iter()
            .map(|(handle, &c)| (handle, Component::Missile(c)));
        let ships = self.ships.iter()
            .map(|(handle, &c)| (handle, Component::Ship(c)));

        bodies
            .chain(crafts)
            .chain(explosions)
            .chain(missiles)
            .chain(ships)
    }

    pub fn item_removed(&mut self) -> events::Source<ItemRemoved> {
        self.item_removed.source()
    }

    pub fn entity_removed(&mut self) -> events::Source<EntityRemoved> {
        self.entity_removed.source()
    }

    pub fn player_created(&mut self) -> events::Source<PlayerCreated> {
        self.player_created.source()
    }
}
