use crate::game::entities::Missile;

events! {
    InEvent {
        LaunchMissile, launch_missile {
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
