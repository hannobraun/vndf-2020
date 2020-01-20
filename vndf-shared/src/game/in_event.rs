events! {
    InEvent {
        ExplosionImminent, explosion_imminent {
            explosion: hecs::Entity,
        }
        RemoveExplosion, remove_explosion {
            explosion: hecs::Entity,
        }
    }
}
