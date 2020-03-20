use crate::game::Game;


pub fn start(_: Game) -> Result<(), Error> {
    Ok(())
}


#[derive(Debug)]
pub enum Error {}
