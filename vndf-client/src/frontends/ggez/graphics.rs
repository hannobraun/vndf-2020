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
        elements::UiElement,
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

use super::{
    draw::{
        draw,
        DrawParam,
    },
    transforms::{
        ScreenTransform,
        WorldTransform,
    },
};


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
        let size_s   = screen_size(context);
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

        let element = UiElement {
            pos:   pos_s,
            angle: -orbit.arg_of_periapsis,
            .. UiElement::default()
        };

        // Draw orbit
        draw(
            context,
            &ScreenTransform { element: &element },
            &ellipse,
            DrawParam::screen(),
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

            let pericenter = UiElement {
                pos: game.state.camera.world_to_screen(
                    size_s,
                    orbit.pericenter,
                ),
                .. UiElement::default()
            };
            let apocenter = UiElement {
                pos: game.state.camera.world_to_screen(
                    size_s,
                    orbit.apocenter,
                ),
                .. UiElement::default()
            };

            draw(
                context,
                &ScreenTransform { element: &pericenter },
                &text(
                    format!(
                        "Periapsis:\nfrom center: {:.0} km\nabove surface:{:.0} km",
                        periapsis_km,
                        periapsis_above_surface_km,
                    )
                ),
                DrawParam::screen(),
            )?;
            draw(
                context,
                &ScreenTransform { element: &apocenter },
                &text(
                    format!(
                        "Apoapsis:\nfrom center: {:.0} km\nabove surface:{:.0} km",
                        apoapsis_km,
                        apoapsis_above_surface_km,
                    )
                ),
                DrawParam::screen(),
            )?;
        }

        Ok(true)
    }

    fn draw_planet(&self, context: &mut Context, planet: &Planet, game: &Game)
        -> GameResult
    {
        draw(
            context,
            &WorldTransform {
                element: &planet.into(),
                camera:  &game.state.camera,
            },
            &self.circle,
            DrawParam::world()
        )?;

        Ok(())
    }

    fn draw_ship(&self, context: &mut Context, ship: &Ship, game: &Game)
        -> GameResult<bool>
    {
        let element = get!(
            UiElement::from_ship(ship, game, screen_size(context))
        );

        draw(
            context,
            &ScreenTransform { element: &element },
            &self.ship,
            DrawParam::screen()
                .color([ship.color[0], ship.color[1], ship.color[2], 1.0]),
        )?;

        let craft = get!(game.state.data.crafts, &ship.craft);
        self.draw_craft_info(context, craft, element, game)?;

        Ok(true)
    }

    fn draw_craft_info(&self,
        context: &mut Context,
        craft:   &Craft,
        element: UiElement,
        game:    &Game,
    )
        -> GameResult<bool>
    {
        let element = UiElement {
            pos: element.pos + graphics::Vec2::new(20.0, -20.0),
            .. UiElement::default()
        };

        let body  = get!(game.state.data.bodies, &craft.body);
        let pos_w = get!(game.state.data.positions, &body.pos);
        let vel   = get!(game.state.data.velocities, &body.vel);

        let pos_km = pos_w.0 / 1000.0;
        let vel_km = vel.0 / 1000.0;

        draw(
            context,
            &ScreenTransform { element: &element },
            &text(
                format!(
                    "Pos: {:.0}/{:.0}\nVel: {:.0}/{:.0} ({:.0})",
                    pos_km.x, pos_km.y,
                    vel_km.x, vel_km.y, vel_km.length(),
                )
            ),
            DrawParam::screen(),
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
            UiElement::from_explosion(explosion, game, screen_size(context))
        );

        let alpha = explosion.strength_left / explosion.strength_total;

        draw(
            context,
            &ScreenTransform { element: &element },
            &self.circle,
            DrawParam::screen()
                .color([1.0, 1.0, 1.0, alpha])
        )?;

        Ok(true)
    }

    fn draw_ui(&self,
        context: &mut Context,
        game:    &Game,
    )
        -> GameResult
    {
        let instructions = format!(
"Instructions:
Turn left - {}
Turn right - {}
Thrust On - {}
Thrust Off - {}
Zoom Camera - Mouse Wheel
End game - {}",
            game.input.config.input.left,
            game.input.config.input.right,
            game.input.config.input.thrust_on,
            game.input.config.input.thrust_off,
            game.input.config.input.quit,
        );

        let element = UiElement {
            pos: graphics::Pnt2::new(20.0, 20.0),
            .. UiElement::default()
        };
        draw(
            context,
            &ScreenTransform { element: &element },
            &text(instructions),
            DrawParam::screen(),
        )?;

        let element = UiElement {
            pos: graphics::Pnt2::new(20.0, 150.0),
            .. UiElement::default()
        };
        draw(
            context,
            &ScreenTransform { element: &element },
            &text(format!("Zoom: {:.3}x", game.input.zoom)),
            DrawParam::screen(),
        )?;

        if game.input.config.diagnostics.frame_time {
            let report = game.state.frame_time.report();
            let frame_time = format!(
                "Frame time:\n{} ms (avg {}/{}/{})",
                report.latest.whole_milliseconds(),
                report.avg_1.whole_milliseconds(),
                report.avg_2.whole_milliseconds(),
                report.avg_3.whole_milliseconds(),
            );

            draw(
                context,
                &ScreenTransform { element: &UiElement::default() },
                &text(frame_time),
                DrawParam::screen()
                    .dest(graphics::Pnt2::new(20.0, 180.0))
            )?;
        }

        if game.input.config.diagnostics.components {
            if let Some(diagnostics) = game.state.diagnostics {
                let diagnostics = format!(
"Components:
Bodies: {}/{}
Crafts: {}/{}
Explosions: {}/{}
Fuels: {}/{}
Healths: {}/{}
Planets: {}/{}
Players: {}/-
Positions: {}/{}
Ships: {}/{}
Velocities: {}/{}
---
Updates per s: {}
Removals per s: {}",
                    diagnostics.bodies, game.state.data.bodies.len(),
                    diagnostics.crafts, game.state.data.crafts.len(),
                    diagnostics.explosions, game.state.data.explosions.len(),
                    diagnostics.fuels, game.state.data.fuels.len(),
                    diagnostics.healths, game.state.data.healths.len(),
                    diagnostics.planets, game.state.data.planets.len(),
                    diagnostics.players,
                    diagnostics.positions, game.state.data.positions.len(),
                    diagnostics.ships, game.state.data.ships.len(),
                    diagnostics.velocities, game.state.data.velocities.len(),
                    game.state.statistics.updates.len(),
                    game.state.statistics.removals.len(),
                );

                draw(
                    context,
                    &ScreenTransform { element: &UiElement::default() },
                    &text(diagnostics),
                    DrawParam::screen()
                        .dest(graphics::Pnt2::new(20.0, 220.0))
                )?;
            }
        }

        if game.input.config.diagnostics.input {
            let mut input_events = String::from("Input:\n");
            for event in game.events.iter().rev() {
                input_events.push_str(&format!("{}\n", event));
            }

            draw(
                context,
                &ScreenTransform { element: &UiElement::default() },
                &text(input_events),
                DrawParam::screen()
                    .dest(graphics::Pnt2::new(20.0, 520.0))
            )?;
        }

        for ship in game.state.data.ships.values() {
            if self.draw_ship_status(context, ship, game)? {
                // There should only be one ship owned by the local player, so
                // let's quit.
                break;
            }
        }

        let element = UiElement {
            size: graphics::Size::new(10.0, 10.0),
            pos:  game.input.pointer_screen,
            .. UiElement::default()
        };

        draw(
            context,
            &ScreenTransform { element: &element },
            &self.pointer,
            DrawParam::screen(),
        )?;

        mouse::set_cursor_hidden(context, true);

        Ok(())
    }

    fn draw_ship_status(&self,
        context: &mut Context,
        ship:    &Ship,
        game:    &Game,
    )
        -> GameResult<bool>
    {
        let craft  = get!(game.state.data.crafts, &ship.craft);
        let fuel   = get!(game.state.data.fuels, &craft.fuel);
        let health = get!(game.state.data.healths, &craft.health);

        if game.state.own_id != Some(craft.owner) {
            return Ok(false);
        }

        let (width, _) = ggez::graphics::drawable_size(context);

        let status = format!("Ship Status
Structural Integrity: {:.2}
Fuel: {:.2}",
            health.value,
            fuel.0,
        );

        draw(
            context,
            &ScreenTransform { element: &UiElement::default() },
            &text(status),
            DrawParam::screen()
                .dest(graphics::Pnt2::new(width - 200.0, 20.0))
        )?;

        Ok(true)
    }
}


fn screen_size(context: &Context) -> graphics::Size {
    let (screen_width, screen_height) = ggez::graphics::drawable_size(context);
    graphics::Size::new(screen_width, screen_height)
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
