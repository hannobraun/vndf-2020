use crate::state::components::{
    Body,
    Engine,
    Ship,
};


pub fn ship() -> (Ship, Body, Engine) {
    (Ship::new(), Body::new(), Engine::new())
}
