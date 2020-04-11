use toadster::store;

use crate::world::{
    math::{
        Angle,
        Pnt2,
        Vec2,
    },
    planets::{
        G,
        Planet,
        Planets,
    },
};

pub struct Orbit {
    pub center:           Pnt2,
    pub eccentricity:     Vec2,
    pub semi_major_axis:  f32,
    pub semi_minor_axis:  f32,
    pub arg_of_periapsis: Angle,
    pub pericenter:       Pnt2,
    pub apocenter:        Pnt2,
    pub periapsis:        f32,
    pub apoapsis:         f32,
    pub ellipse_pos:      Pnt2,
}

impl Orbit {
    pub fn from_state_vectors(
        pos:     Pnt2,
        vel:     Vec2,
        planets: &Planets<impl for<'r> store::Values<'r, Planet>>,
    )
        -> Option<Self>
    {
        let planet = planets.dominant_at(pos);

        // State vectors
        let r = pos - planet.pos;
        let v = vel;

        // Standard gravitational parameter
        let mu = G * planet.mass;

        // Orbital eccentricity
        let e =
            r * (v.length().powi(2) / mu - 1.0 / r.length())
            - v * r.dot(v) / mu;

        // What we computed here is the eccentricity vector. It's magnitude is
        // the eccentricity. The eccentricity tells us what kind of orbit we're
        // dealing with:
        // |e| == 0    => Circular
        // 0 < |e| < 1 => Elliptical
        // |e| == 1:   => Parabolic
        // |e| > 1:    => Hyperbolic

        // For now, we're only dealing with circular and elliptical orbits.
        if e.length() >= 1.0 {
            return None;
        }

        // Specific orbital energy
        let ep = v.length().powi(2) / 2.0 - mu / r.length();

        // Semi-major axis
        let a = -(mu / 2.0 / ep);

        // Semi-minor axis
        let b = a * (1.0 - e.length().powi(2)).sqrt();

        // Argument of periapsis
        let w = f32::atan2(e.y, e.x);

        // Pericenter (point of closest approach)
        let pericenter = planet.pos + e.normalize() * (1.0 - e.length()) * a;
        let periapsis  = (pericenter - planet.pos).length();

        // Apocenter (farthest point of orbit)
        let apocenter = pericenter - e.normalize() * 2.0 * a;
        let apoapsis  = (apocenter - planet.pos).length();

        // Center of ellipse
        let ellipse_pos = pericenter - e.normalize() * a;

        Some(
            Self {
                center:           planet.pos,
                eccentricity:     e,
                semi_major_axis:  a,
                semi_minor_axis:  b,
                arg_of_periapsis: Angle::radians(w),
                pericenter,
                apocenter,
                periapsis,
                apoapsis,
                ellipse_pos,
            }
        )
    }
}
