use hecs::Entity;


events!(
    OutEvent {
        Despawn, despawn {
            entity: Entity,
        }
    }
);
