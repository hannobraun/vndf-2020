use ggez::{
    Context,
    GameResult,
    graphics::{
        DrawMode,
        Mesh,
        Text,
    },
    input::mouse,
};
use toadster::{
    handle,
    store,
};

use crate::{
    draw::{
        draw,
        DrawParam,
    },
    game::Game,
    graphics::{
        self,
        elements::UiElement,
        vertices,
    },
    shared::world::{
        self,
        behavior::{
            crafts::Craft,
            explosions::Explosion,
            missiles::Missile,
            physics::Body,
            orbits::Orbit,
            planets::{
                Planet,
                Planets,
            },
            ships::Ship,
        },
    },
};

use super::transforms::{
    ScreenTransform,
    WorldTransform,
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
    square:   Mesh,
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
        let square = Mesh::new_polygon(
            context,
            DrawMode::fill(),
            vertices::SQUARE,
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
                square,
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
        for planet in game.state.data.planets.values() {
            self.draw_planet(context, planet, game)?;
        }
        for ship in game.state.data.ships.values() {
            self.draw_ship(context, ship, game)?;
        }
        for missile in game.state.data.missiles.values() {
            self.draw_missile(context, missile, game)?;
        }
        for explosion in game.state.data.explosions.values() {
            self.draw_explosion(context, explosion, game)?;
        }

        Ok(())
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
        let craft = get!(game.state.data.crafts, &ship.craft);

        self.draw_orbit(context, &craft.body, ship.color, game)?;

        let element = get!(
            UiElement::from_ship(ship, game, screen_size(context))
        );

        draw(
            context,
            &ScreenTransform,
            &self.ship,
            DrawParam::screen()
                .dest(element.pos)
                .rotation(element.angle.radians)
                .scale(element.size)
                .color([ship.color[0], ship.color[1], ship.color[2], 1.0]),
        )?;

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
        let element = element.offset(graphics::Vec2::new(20.0, -20.0));

        let body  = get!(game.state.data.bodies, &craft.body);
        let pos_w = get!(game.state.data.positions, &body.pos);
        let vel   = get!(game.state.data.velocities, &body.vel);

        let pos_km = pos_w.0 / 1000.0;
        let vel_km = vel.0 / 1000.0;

        draw(
            context,
            &ScreenTransform,
            &Text::new(
                format!(
                    "Pos: {:.0}/{:.0}\nVel: {:.0}/{:.0} ({:.0})",
                    pos_km.x, pos_km.y,
                    vel_km.x, vel_km.y, vel_km.length(),
                )
            ),
            DrawParam::screen()
                .dest(element.pos),
        )?;

        Ok(true)
    }

    fn draw_missile(&self,
        context: &mut Context,
        missile: &Missile,
        game:    &Game,
    )
        -> GameResult<bool>
    {
        let element = get!(
            UiElement::from_missile(missile, game, screen_size(context))
        );

        draw(
            context,
            &ScreenTransform,
            &self.square,
            DrawParam::screen()
                .dest(element.pos)
                .scale(element.size)
        )?;

        self.draw_missile_target_line(context, missile, element, game)?;

        Ok(true)
    }

    fn draw_missile_target_line(&self,
        context: &mut Context,
        missile: &Missile,
        element: UiElement,
        game:    &Game,
    )
        -> GameResult<bool>
    {
        let target = get!(game.state.data.targets, &missile.target);

        let target = game.state.camera.world_to_screen(
            screen_size(context),
            target.value,
        );

        let line = Mesh::new_line(
            context,
            &[element.pos, target],
            1.5,
            [0.0, 1.0, 0.0, 1.0].into(),
        )?;

        draw(
            context,
            &ScreenTransform,
            &line,
            DrawParam::screen(),
        )?;

        Ok(true)
    }

    fn draw_orbit(&self,
        context: &mut Context,
        body:    impl Into<handle::Weak<Body>>,
        color:   [f32; 3],
        game:    &Game,
    )
        -> GameResult<bool>
    {
        let mut body = get!(game.state.data.bodies, body).clone();
        body.acc = world::Vec2::zero();

        let pos = *get!(game.state.data.positions,  &body.pos);
        let vel = *get!(game.state.data.velocities, &body.vel);

        let planets = Planets(&game.state.data.planets);

        let orbit = match Orbit::from_state_vectors(pos.0, vel.0, &planets) {
            Some(orbit) => orbit,
            None        => return Ok(true),
        };

        let size_s           = screen_size(context);
        let pixels_per_meter = game.state.camera.pixels_per_meter(size_s);

        // Ellipse in screen coordinates
        let pos_s = game.state.camera.world_to_screen(
            size_s,
            orbit.ellipse_pos,
        );
        let r1_s = orbit.semi_major_axis * pixels_per_meter;
        let r2_s = orbit.semi_minor_axis * pixels_per_meter;

        let ellipse = Mesh::new_ellipse(
            context,
            DrawMode::stroke(2.0),
            [0.0, 0.0],
            r1_s.0,
            r2_s.0,
            0.5,
            [color[0], color[1], color[2], 0.5].into(),
        )?;

        // Draw orbit
        draw(
            context,
            &ScreenTransform,
            &ellipse,
            DrawParam::screen()
                .dest(pos_s)
                .rotation(orbit.arg_of_periapsis.radians),
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

            draw(
                context,
                &ScreenTransform,
                &Text::new(
                    format!(
                        "Periapsis:\nfrom center: {:.0} km\nabove surface:{:.0} km",
                        periapsis_km,
                        periapsis_above_surface_km,
                    )
                ),
                DrawParam::screen()
                    .dest(
                        game.state.camera.world_to_screen(
                            size_s,
                            orbit.pericenter,
                        )
                    ),
            )?;
            draw(
                context,
                &ScreenTransform,
                &Text::new(
                    format!(
                        "Apoapsis:\nfrom center: {:.0} km\nabove surface:{:.0} km",
                        apoapsis_km,
                        apoapsis_above_surface_km,
                    )
                ),
                DrawParam::screen()
                    .dest(
                        game.state.camera.world_to_screen(
                            size_s,
                            orbit.apocenter,
                        )
                    ),
            )?;
        }

        Ok(true)
    }

    fn draw_explosion(&self,
        context:   &mut Context,
        explosion: &Explosion,
        game:      &Game,
    )
        -> GameResult<bool>
    {
        let pos = get!(game.state.data.positions, &explosion.pos);

        let alpha = explosion.strength_left / explosion.strength_total;
        let size  = explosion.strength_total * 2.0;

        let pos = game.state.camera.world_to_screen(
            screen_size(context),
            pos.0,
        );

        draw(
            context,
            &ScreenTransform,
            &self.circle,
            DrawParam::screen()
                .dest(pos)
                .scale(graphics::Vec2::new(size, size))
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
Shoot - {} (aim with mouse)
Zoom Camera - Mouse Wheel
End game - {}",
            game.input.config.input.left,
            game.input.config.input.right,
            game.input.config.input.thrust_on,
            game.input.config.input.thrust_off,
            game.input.config.input.launch,
            game.input.config.input.quit,
        );

        draw(
            context,
            &ScreenTransform,
            &Text::new(instructions),
            DrawParam::screen()
                .dest(graphics::Pnt2::new(20.0, 20.0))
        )?;

        draw(
            context,
            &ScreenTransform,
            &Text::new(format!("Zoom: {:.3}x", game.input.zoom)),
            DrawParam::screen()
                .dest(graphics::Pnt2::new(20.0, 150.0)),
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
                &ScreenTransform,
                &Text::new(frame_time),
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
Guidances: {}/-
Healths: {}/{}
Planets: {}/{}
Players: {}/-
Missiles: {}/{}
Positions: {}/{}
Ships: {}/{}
Targets: {}/{}
Velocities: {}/{}
---
Updates per s: {}
Removals per s: {}",
                    diagnostics.bodies, game.state.data.bodies.len(),
                    diagnostics.crafts, game.state.data.crafts.len(),
                    diagnostics.explosions, game.state.data.explosions.len(),
                    diagnostics.fuels, game.state.data.fuels.len(),
                    diagnostics.guidances,
                    diagnostics.healths, game.state.data.healths.len(),
                    diagnostics.planets, game.state.data.planets.len(),
                    diagnostics.players,
                    diagnostics.missiles, game.state.data.missiles.len(),
                    diagnostics.positions, game.state.data.positions.len(),
                    diagnostics.ships, game.state.data.ships.len(),
                    diagnostics.targets, game.state.data.targets.len(),
                    diagnostics.velocities, game.state.data.velocities.len(),
                    game.state.statistics.updates.len(),
                    game.state.statistics.removals.len(),
                );

                draw(
                    context,
                    &ScreenTransform,
                    &Text::new(diagnostics),
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
                &ScreenTransform,
                &Text::new(input_events),
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

        draw(
            context,
            &ScreenTransform,
            &self.pointer,
            DrawParam::screen()
                .dest(game.input.pointer_screen)
                .scale(graphics::Vec2::new(10.0, 10.0))
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
Fuel: {:.2}
Heavy Missiles: {}",
            health.value,
            fuel.0,
            ship.missiles,
        );

        draw(
            context,
            &ScreenTransform,
            &Text::new(status),
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
