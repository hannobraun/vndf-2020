use ggez::{
    Context,
    GameResult,
    graphics::{
        DrawMode,
        Mesh,
        Text,
        TextFragment,
    },
    input::mouse,
};
use toadster::{
    handle,
    store,
};

use crate::{
    game::Game,
    graphics::{
        self,
        elements::{
            ScreenElement,
            WorldElement,
        },
        screen::Screen,
        ui,
        vertices,
    },
    shared::world::behavior::{
        crafts::Craft,
        explosions::Explosion,
        orbits::Orbit,
        planets::Planet,
        ships::Ship,
    },
};

use super::draw::draw;


macro_rules! get {
    ($store:expr, $handle:expr) => {
        get!($store.get($handle))
    };
    ($opt:expr) => {
        match $opt {
            Some(value) => value,
            None        => return Ok(false),
        }
    };
}


pub struct Graphics {
    circle:   Mesh,
    ship:     Mesh,
    pointer:  Mesh,
}

impl Graphics {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let circle = Mesh::new_circle(
            context,
            DrawMode::fill(),
            [0.0, 0.0],
            1.0,
            0.01,
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;
        let ship = Mesh::new_polygon(
            context,
            DrawMode::fill(),
            vertices::SHIP,
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;
        let pointer = Mesh::new_polygon(
            context,
            DrawMode::stroke(0.2),
            vertices::POINTER,
            [1.0, 0.0, 0.0, 1.0].into(),
        )?;

        Ok(
            Graphics {
                circle,
                ship,
                pointer,
            }
        )
    }

    pub fn draw(&self,
        context: &mut Context,
        game:    &Game,
    )
        -> GameResult
    {
        let c = graphics::BACKGROUND_COLOR;
        let c = [c.r as f32, c.g as f32, c.b as f32, c.a as f32];
        ggez::graphics::clear(context, c.into());

        self.draw_world(context, game)?;
        self.draw_ui(context, game)?;

        ggez::graphics::present(context)?;
        Ok(())
    }

    fn draw_world(&self,
        context: &mut Context,
        game:    &Game,
    )
        -> GameResult
    {
        for orbit in game.state.active_orbits() {
            self.draw_orbit(context, &orbit, game)?;
        }
        for planet in game.state.data.planets.values() {
            self.draw_planet(context, planet, game)?;
        }
        for ship in game.state.data.ships.values() {
            self.draw_ship(context, ship, game)?;
        }
        for explosion in game.state.data.explosions.values() {
            self.draw_explosion(context, explosion, game)?;
        }

        Ok(())
    }

    fn draw_orbit(&self,
        context: &mut Context,
        orbit:   &Orbit,
        game:    &Game,
    )
        -> GameResult<bool>
    {
        let size_s   = screen(context).size;
        let pi_per_m = game.state.camera.pixels_per_meter(size_s);

        // Ellipse in screen coordinates
        let pos_s = game.state.camera.world_to_screen(
            size_s,
            orbit.ellipse_pos,
        );
        let r1_s = orbit.semi_major_axis * pi_per_m;
        let r2_s = orbit.semi_minor_axis * pi_per_m;

        let ellipse = Mesh::new_ellipse(
            context,
            DrawMode::stroke(2.0),
            [0.0, 0.0],
            r1_s.0,
            r2_s.0,
            0.5,
            [1.0, 1.0, 1.0, 0.5].into(),
        )?;

        let transform =
            ScreenElement {
                pos:   pos_s,
                angle: -orbit.arg_of_periapsis,
                .. ScreenElement::default()
            }
            .transform(screen(context).size)
            .to_native();

        // Draw orbit
        draw(
            context,
            transform,
            &ellipse,
            None,
        )?;

        // Display periapsis and apoapsis
        //
        // If our orbit is nearly circular, the computed apses will jump around
        // like crazy. Let's make sure we have a minimum of eccentricity, so
        // they become well-defined.
        if orbit.eccentricity.length() > 0.01 {
            let periapsis_km = orbit.periapsis / 1000.0;
            let apoapsis_km  = orbit.apoapsis  / 1000.0;

            let periapsis_above_surface_km =
                orbit.periapsis_above_surface / 1000.0;
            let apoapsis_above_surface_km =
                orbit.apoapsis_above_surface / 1000.0;

            let transform =
                ScreenElement {
                    pos: game.state.camera.world_to_screen(
                        size_s,
                        orbit.pericenter,
                    ),
                    .. ScreenElement::default()
                }
                .transform(screen(context).size)
                .to_native();
            draw(
                context,
                transform,
                &text(
                    format!(
                        "Periapsis:\nfrom center: {:.0} km\nabove surface:{:.0} km",
                        periapsis_km,
                        periapsis_above_surface_km,
                    )
                ),
                None,
            )?;

            let transform =
                ScreenElement {
                    pos: game.state.camera.world_to_screen(
                        size_s,
                        orbit.apocenter,
                    ),
                    .. ScreenElement::default()
                }
                .transform(screen(context).size)
                .to_native();
            draw(
                context,
                transform,
                &text(
                    format!(
                        "Apoapsis:\nfrom center: {:.0} km\nabove surface:{:.0} km",
                        apoapsis_km,
                        apoapsis_above_surface_km,
                    )
                ),
                None,
            )?;
        }

        Ok(true)
    }

    fn draw_planet(&self, context: &mut Context, planet: &Planet, game: &Game)
        -> GameResult
    {
        let element: WorldElement = planet.into();
        let transform = element
            .transform(&game.state.camera, screen(context).size)
            .to_native();

        draw(
            context,
            transform,
            &self.circle,
            None,
        )?;

        Ok(())
    }

    fn draw_ship(&self, context: &mut Context, ship: &Ship, game: &Game)
        -> GameResult<bool>
    {
        let element = get!(
            ScreenElement::from_ship(ship, game, &screen(context))
        );
        let transform = element
            .transform(screen(context).size)
            .to_native();

        draw(
            context,
            transform,
            &self.ship,
            Some([ship.color[0], ship.color[1], ship.color[2], 1.0]),
        )?;

        let craft = get!(game.state.data.crafts, &ship.craft);
        self.draw_craft_info(context, craft, element, game)?;

        Ok(true)
    }

    fn draw_craft_info(&self,
        context: &mut Context,
        craft:   &Craft,
        element: ScreenElement,
        game:    &Game,
    )
        -> GameResult<bool>
    {
        let transform =
            ScreenElement {
                pos: element.pos + graphics::Vec2::new(20.0, -20.0),
                .. ScreenElement::default()
            }
            .transform(screen(context).size)
            .to_native();

        let body  = get!(game.state.data.bodies, &craft.body);
        let pos_w = get!(game.state.data.positions, &body.pos);
        let vel   = get!(game.state.data.velocities, &body.vel);

        let pos_km = pos_w.0 / 1000.0;
        let vel_km = vel.0 / 1000.0;

        draw(
            context,
            transform,
            &text(
                format!(
                    "Pos: {:.0}/{:.0}\nVel: {:.0}/{:.0} ({:.0})",
                    pos_km.x, pos_km.y,
                    vel_km.x, vel_km.y, vel_km.length(),
                )
            ),
            None,
        )?;

        Ok(true)
    }

    fn draw_explosion(&self,
        context:   &mut Context,
        explosion: &Explosion,
        game:      &Game,
    )
        -> GameResult<bool>
    {
        let element = get!(
            ScreenElement::from_explosion(explosion, game, &screen(context))
        );
        let transform = element
            .transform(screen(context).size)
            .to_native();

        let alpha = explosion.strength_left / explosion.strength_total;

        draw(
            context,
            transform,
            &self.circle,
            Some([1.0, 1.0, 1.0, alpha])
        )?;

        Ok(true)
    }

    fn draw_ui(&self,
        context: &mut Context,
        game:    &Game,
    )
        -> GameResult
    {
        for element in ui::elements(game, &screen(context)) {
            draw(
                context,
                element.transform(&screen(context)),
                &text(element.text),
                None,
            )?;
        }

        let transform =
            ScreenElement {
                size: graphics::Size::new(10.0, 10.0),
                pos:  game.input.pointer_screen,
                .. ScreenElement::default()
            }
            .transform(screen(context).size)
            .to_native();

        draw(
            context,
            transform,
            &self.pointer,
            None,
        )?;

        mouse::set_cursor_hidden(context, true);

        Ok(())
    }
}


fn screen(context: &Context) -> Screen {
    let (width, height) = ggez::graphics::drawable_size(context);

    Screen {
        size: graphics::Size::new(width, height),
        scale_factor: 1.0,
    }
}


struct OneStore<T> {
    pub handle: handle::Weak<T>,
    pub data:   T
}

impl<T> store::Get<T> for OneStore<T> {
    fn get(&self, handle: impl Into<handle::Weak<T>>) -> Option<&T> {
        if handle.into() == self.handle {
            Some(&self.data)
        }
        else {
            None
        }
    }
}

impl<T> store::GetMut<T> for OneStore<T> {
    fn get_mut(&mut self, handle: impl Into<handle::Weak<T>>)
        -> Option<&mut T>
    {
        if handle.into() == self.handle {
            Some(&mut self.data)
        }
        else {
            None
        }
    }
}


fn text(s: impl AsRef<str>) -> Text {
    Text::new(
        TextFragment::new(s.as_ref())
    )
}
