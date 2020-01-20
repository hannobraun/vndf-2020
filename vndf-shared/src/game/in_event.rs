events! {
    InEvent {
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
