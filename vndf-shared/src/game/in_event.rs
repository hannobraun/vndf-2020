use hecs::Entity;

use crate::game::entities::Missile;

events! {
    InEvent {
        LaunchMissile, launch_missile {
            missile: Missile,
        }
        DeadEntity, dead_entity {
            entity: Entity,
        }
        Explosion, explosion {
            explosion: Entity,
        }
        RemoveExplosion, remove_explosion {
            explosion: Entity,
        }
    }
}
