pub struct Player {
    pub id: u64,
}

impl Player {
    pub fn new(id: u64) -> Self {
        Self {
            id,
        }
    }
}
