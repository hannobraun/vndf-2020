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
    game::{
        crafts::Craft,
        missiles::MissileLaunch,
        physics::Body,
        players::PlayerId,
        ships::{
            Ship,
            ShipEntity,
        },
    },
    input,
    world,
};

use super::{
    components::Player,
    events::PlayerCreated,
};


pub fn connect_player(
    world:          &mut world::Spawn,
    crafts:         &mut Store<Craft>,
    players:        &mut Store<Player>,
    ships:          &mut Store<Ship>,
    player_created: &mut events::Sink<PlayerCreated>,
    index:          &mut HashMap<SocketAddr, Handle>,
    id:             PlayerId,
    addr:           SocketAddr,
    color:          [f32; 3],
) {
    let handle = players.insert(Player::new(id, addr));
    index.insert(addr, handle);

    ShipEntity { owner: id, color }.create(world, crafts, ships);
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
    world:          world::Query,
    crafts:         &mut Store<Craft>,
    players:        &Store<Player>,
    ships:          &mut Store<Ship>,
    missile_launch: &mut events::Sink<MissileLaunch>,
    index:          &mut HashMap<SocketAddr, Handle>,
    address:        SocketAddr,
    input:          input::Event,
)
    -> Option<()>
{
    let player = index.get(&address)?;
    let player = players.get(*player)?;

    for ship in ships.values_mut() {
        let entity = hecs::Entity::from_bits(ship.entity);

        let body  = world.get::<Body>(entity);
        let craft = crafts.get_mut(ship.craft);

        if let (Ok(body), Some(mut craft)) = (body, craft) {
            if craft.owner != player.id {
                continue;
            }

            match input {
                input::Event::Rotate(rotation) => {
                    ship.rotation = rotation;
                }
                input::Event::Thrust(thrust) => {
                    craft.engine_on = thrust;
                }
                input::Event::LaunchMissile { target } => {
                    let missile = ship.launch_missile(
                        craft.owner,
                        &body,
                        target,
                    );
                    if let Some(missile) = missile {
                        missile_launch.push(MissileLaunch { missile });
                    }
                }
            }
        }
    }

    Some(())
}
