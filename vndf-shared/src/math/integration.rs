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
    velocity_verlet(dt, pos, vel, acc)
}


/// Semi-implicit Euler method
///
/// Pretty simple, but doesn't provide good enough accuracy when integrating
/// with a bigger time step, as is done when computing the projected path. As a
/// result, the projected path is inaccurate, and varies according to the
/// current position in the orbit.
pub fn semi_implicit_euler(
    dt:  f32,
    pos: &mut Pnt2,
    vel: &mut Vec2,
    acc: impl Fn(Pnt2) -> Vec2,
) {
    *vel += acc(*pos) * dt;
    *pos += *vel * dt;
}

/// Velocity Verlet method
///
/// A bit more complex than semi-implicit Euler, but much more accurate (it's a
/// second-order method, while semi-implicit Euler is first-order, whatever that
/// actually means).
///
/// In principle, it shows the same problems as semi-implicit Euler in regards
/// to the path projection (as can be expected), but the improved accuracy makes
/// it good enough for now.
pub fn velocity_verlet(
    dt:  f32,
    pos: &mut Pnt2,
    vel: &mut Vec2,
    acc: impl Fn(Pnt2) -> Vec2,
) {
    let acc_t = acc(*pos);
    *pos += *vel * dt + 0.5 * acc_t * dt*dt;
    let acc_t_plus_dt = acc(*pos);
    *vel += (acc_t + acc_t_plus_dt) * 0.5 * dt;
}


// I found a helpful page that lists a bunch of symplectic integrators:
// https://docs.juliadiffeq.org/latest/solvers/dynamical_solve/#Symplectic-Integrators-1
//
// The list on that page has a not for each of the integrators listed. If more
// accuracy is desired, it might make sense to go through the higher-order
// integrators in that list, and see if they can be applied here.
