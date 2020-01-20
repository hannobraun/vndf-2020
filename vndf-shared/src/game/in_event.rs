events! {
    InEvent {
        Explosion, explosion {
            explosion: hecs::Entity,
        }
        RemoveExplosion, remove_explosion {
            explosion: hecs::Entity,
        }
    }
}
