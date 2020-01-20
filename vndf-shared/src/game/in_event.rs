use crate::game::entities::Missile;

events! {
    InEvent {
        MissileLaunch, missile_launch {
            missile: Missile,
        }
        DeadEntity, dead_entity {
            entity: hecs::Entity,
        }
        Explosion, explosion {
            explosion: hecs::Entity,
        }
        RemoveExplosion, remove_explosion {
            explosion: hecs::Entity,
        }
    }
}
