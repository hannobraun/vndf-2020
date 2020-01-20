events! {
    InEvent {
        Death, death {
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
