use crate::math::{
    Pnt2,
    Vec2,
};


pub fn integrate(dt: f32, pos: &mut Pnt2, vel: &mut Vec2, acc: &mut Vec2) {
    semi_implicit_euler(dt, pos, vel, acc)
}

/// Semi-implicit Euler method
pub fn semi_implicit_euler(
    dt:  f32,
    pos: &mut Pnt2,
    vel: &mut Vec2,
    acc: &mut Vec2,
) {
    *vel += *acc * dt;
    *pos += *vel * dt;
}
