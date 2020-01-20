events! {
    InEvent {
        EntityDead, entity_dead {
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
