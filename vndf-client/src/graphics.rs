use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        DrawMode,
        DrawParam,
        Mesh,
        Text,
    },
    input::mouse,
};

use crate::{
    game::State,
    input::Input,
    shared::{
        game::{
            WORLD_SIZE,
            explosions::Explosion,
            missiles::Missile,
            ships::Ship,
        },
        math::{
            prelude::*,
            Vec2,
        }
    },
    transforms,
};


pub struct Graphics {
    boundary:  Mesh,
    explosion: Mesh,
    missile:   Mesh,
    ship:      Mesh,

    pointer: Mesh,
}

impl Graphics {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let boundary = Mesh::new_polygon(
            context,
            DrawMode::stroke(3.0 / WORLD_SIZE),
            &[
                [ 0.5,  0.5],
                [ 0.5, -0.5],
                [-0.5, -0.5],
                [-0.5,  0.5],
            ],
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;
        let explosion = Mesh::new_circle(
            context,
            DrawMode::fill(),
            [0.0, 0.0],
            1.0,
            0.01,
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;
        let missile = Mesh::new_polygon(
            context,
            DrawMode::fill(),
            &[
                [ 0.5,  0.5],
                [ 0.5, -0.5],
                [-0.5, -0.5],
                [-0.5,  0.5],
            ],
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;
        let ship = Mesh::new_polygon(
            context,
            DrawMode::fill(),
            &[
                [ 0.6,  0.0],
                [-0.4,  0.4],
                [-0.1,  0.0],
                [-0.4, -0.4],
            ],
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;

        let pointer = Mesh::new_polygon(
            context,
            DrawMode::stroke(0.2),
            &[
                [ 0.5,  0.5],
                [ 0.0, -0.5],
                [-0.5,  0.5],
            ],
            [1.0, 0.0, 0.0, 1.0].into(),
        )?;

        Ok(
            Graphics {
                boundary,
                explosion,
                missile,
                ship,

                pointer,
            }
        )
    }

    pub fn draw(&self,
        context: &mut Context,
        input:   &Input,
        state:   &State,
    )
        -> GameResult
    {
        graphics::clear(context, [0.0, 0.0, 0.15, 1.0].into());

        self.draw_world(context, state)?;
        self.draw_ui(context, input, state)?;

        graphics::present(context)?;
        Ok(())
    }

    fn draw_world(&self, context: &mut Context, state: &State) -> GameResult {
        transforms::activate_world_coordinate_system(context)?;

        self.draw_boundary(context)?;

        for ship in state.ships.values() {
            self.draw_ship(context, ship, state)?;
        }

        for (_, missile) in &state.missiles {
            self.draw_missile(context, missile, state)?;
        }

        for (_, explosion) in &state.explosions {
            self.draw_explosion(context, explosion, state)?;
        }

        Ok(())
    }

    fn draw_boundary(&self, context: &mut Context) -> GameResult {
        graphics::draw(
            context,
            &self.boundary,
            DrawParam::new()
                .scale([WORLD_SIZE, WORLD_SIZE])
        )
    }

    fn draw_ship(&self, context: &mut Context, ship: &Ship, state: &State)
        -> GameResult
    {
        let body = state.crafts.get(ship.craft)
            .and_then(|craft| state.bodies.get(craft.body));

        if let Some(body) = body {
            graphics::draw(
                context,
                &self.ship,
                DrawParam::new()
                    .dest(body.pos)
                    .rotation(Vec2::unit_x().angle(body.dir).0)
                    .scale([30.0, 30.0])
                    .color(
                        [ship.color[0], ship.color[1], ship.color[2], 1.0]
                            .into(),
                    ),
            )?
        }

        Ok(())
    }

    fn draw_missile(&self,
        context: &mut Context,
        missile: &Missile,
        state:   &State,
    )
        -> GameResult
    {
        let body = state.crafts.get(missile.craft)
            .and_then(|craft| state.bodies.get(craft.body));

        if let Some(body) = body {
            graphics::draw(
                context,
                &self.missile,
                DrawParam::new()
                    .dest(body.pos)
                    .scale([4.0, 4.0])
            )?;

            let line = Mesh::new_line(
                context,
                &[body.pos, missile.target],
                1.5,
                [0.0, 1.0, 0.0, 1.0].into(),
            )?;

            graphics::draw(
                context,
                &line,
                DrawParam::new(),
            )?;
        }

        Ok(())
    }

    fn draw_explosion(&self,
        context:   &mut Context,
        explosion: &Explosion,
        state:     &State,
    )
        -> GameResult
    {
        if let Some(body) = state.bodies.get(explosion.body) {
            let alpha = explosion.strength_left / explosion.strength_total;
            let size  = explosion.strength_total * 2.0;

            graphics::draw(
                context,
                &self.explosion,
                DrawParam::new()
                    .dest(body.pos)
                    .scale([size, size])
                    .color([1.0, 1.0, 1.0, alpha].into())
            )?;
        }

        Ok(())
    }

    fn draw_ui(&self,
        context: &mut Context,
        input:   &Input,
        state:   &State,
    )
        -> GameResult
    {
        transforms::activate_ui_coordinate_system(context)?;

        let instructions = format!(
"Instructions:
Turn left - {}
Turn right - {}
Accelerate - {}
Shoot - {} (aim with mouse)
End game - Escape",
            input.config.input.left,
            input.config.input.right,
            input.config.input.thrust,
            input.config.input.launch,
        );

        graphics::draw(
            context,
            &Text::new(instructions),
            DrawParam::new()
                .dest([20.0, 20.0])
        )?;

        if let Some(diagnostics) = state.diagnostics {
            let diagnostics = format!(
"Diagnostics:
Bodies: {}/{}
Crafts: {}/{}
Explosions: {}/{}
Healths: {}/{}
Players: {}/-
Missiles: {}/{}
Ships: {}/{}
---
Updates per s: {}
Removals per s: {}",
                diagnostics.num_bodies, state.bodies.len(),
                diagnostics.num_crafts, state.crafts.len(),
                diagnostics.num_explosions, state.explosions.len(),
                diagnostics.num_healths, state.healths.len(),
                diagnostics.num_players,
                diagnostics.num_missiles, state.missiles.len(),
                diagnostics.num_ships, state.ships.len(),
                state.statistics.updates.len(),
                state.statistics.removals.len(),
            );

            graphics::draw(
                context,
                &Text::new(diagnostics),
                DrawParam::new()
                    .dest([20.0, 220.0])
            )?;
        }

        for ship in state.ships.values() {
            self.draw_ship_status(context, ship, state)?;

            // There should only be one ship owned by the local player, so let's
            // quit.
            break;
        }

        if input.pointer_world.is_some() {
            graphics::draw(
                context,
                &self.pointer,
                DrawParam::new()
                    .dest(input.pointer_screen)
                    .scale([10.0, 10.0])
            )?;
        }

        mouse::set_cursor_hidden(context, input.pointer_world.is_some());

        Ok(())
    }

    fn draw_ship_status(&self,
        context: &mut Context,
        ship:    &Ship,
        state:   &State,
    )
        -> GameResult
    {
        let craft = match state.crafts.get(ship.craft) {
            Some(craft) => craft,
            None        => return Ok(()),
        };

        let health = match state.healths.get(craft.health) {
            Some(health) => health,
            None         => return Ok(()),
        };

        if state.own_id != Some(craft.owner) {
            return Ok(());
        }

        let (width, _) = graphics::drawable_size(context);

        let status = format!("Ship Status
Structural Integrity: {:.2}
Fuel: {:.2}
Heavy Missiles: {}",
            health.value,
            craft.fuel,
            ship.missiles,
        );

        graphics::draw(
            context,
            &Text::new(status),
            DrawParam::new()
                .dest([width - 200.0, 20.0])
        )
    }
}
