/// Collection of symplectic numerical integrators
///
/// Only one is used in the game, but the others are kept around for quick
/// comparison. Since orbital mechanics are being simulated, only symplectic
/// integrators are useful.


use crate::math::{
    Pnt2,
    Vec2,
};


pub fn integrate(
    dt:  f32,
    pos: &mut Pnt2,
    vel: &mut Vec2,
    acc: impl Fn(Pnt2) -> Vec2,
) {
    semi_implicit_euler(dt, pos, vel, acc)
}

/// Semi-implicit Euler method
pub fn semi_implicit_euler(
    dt:  f32,
    pos: &mut Pnt2,
    vel: &mut Vec2,
    acc: impl Fn(Pnt2) -> Vec2,
) {
    *vel += acc(*pos) * dt;
    *pos += *vel * dt;
}
