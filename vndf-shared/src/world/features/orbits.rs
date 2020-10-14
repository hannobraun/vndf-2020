use toadster::store;

use crate::world::{
    math::{Angle, Length, Pnt2, Scalar, Vec2},
    planets::{Planet, Planets, G},
};

pub struct Orbit {
    pub center: Pnt2,
    pub eccentricity: Vec2,
    pub semi_major_axis: Length,
    pub semi_minor_axis: Length,
    pub arg_of_periapsis: Angle,
    pub periapsis: Apsis,
    pub apoapsis: Apsis,
    pub ellipse_pos: Pnt2,
    pub orbiter: Orbiter,
}

impl Orbit {
    pub fn new(
        orbiter: Orbiter,
        planets: &Planets<impl for<'r> store::Values<'r, Planet>>,
    ) -> Option<Self> {
        let planet = planets.dominant_at(orbiter.pos);

        // State vectors
        let r = orbiter.pos - planet.pos;
        let v = orbiter.vel;

        // Standard gravitational parameter
        let mu = G * planet.mass;

        // Orbital eccentricity
        //
        // The eccentricity vector points from apocenter to pericenter.
        let e = r * (v.length().powi(2) / mu - 1.0 / r.length()) - v * r.dot(v) / mu;

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
        let w = -Scalar::atan2(e.y, e.x);

        // Pericenter (point of closest approach)
        let periapsis = planet.pos + e.normalize() * (1.0 - e.length()) * a;
        let periapsis = Apsis::new(periapsis, planet);

        // Apocenter (farthest point of orbit)
        let apoapsis = periapsis.position - e.normalize() * 2.0 * a;
        let apoapsis = Apsis::new(apoapsis, planet);

        // Center of ellipse
        let ellipse_pos = periapsis.position - e.normalize() * a;

        Some(Self {
            center: planet.pos,
            eccentricity: e,
            semi_major_axis: Length::new(a),
            semi_minor_axis: Length::new(b),
            arg_of_periapsis: Angle::radians(w),
            periapsis,
            apoapsis,
            ellipse_pos,
            orbiter,
        })
    }
}

pub struct Orbiter {
    pub pos: Pnt2,
    pub vel: Vec2,
}

pub struct Apsis {
    pub position: Pnt2,
    pub distance: Length,
    pub from_surface: Length,
}

impl Apsis {
    pub fn new(position: Pnt2, planet: &Planet) -> Self {
        let distance = Length::new((position - planet.pos).length());
        let from_surface = distance - planet.radius;

        Self {
            position,
            distance,
            from_surface,
        }
    }
}
