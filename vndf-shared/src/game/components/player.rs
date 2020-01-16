use crate::game::PlayerId;


#[derive(Clone, Copy)]
pub struct Player {
    pub id: PlayerId,
}

impl Player {
    pub fn new(id: PlayerId) -> Self {
        Self {
            id,
        }
    }
}
