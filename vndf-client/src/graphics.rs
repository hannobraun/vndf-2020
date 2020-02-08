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


macro_rules! get {
    ($store:expr, $handle:expr) => {
        match $store.get($handle) {
            Some(value) => value,
            None        => return Ok(false),
        }
    };
}


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

        for ship in state.data.ships.values() {
            self.draw_ship(context, ship, state)?;
        }

        for (_, missile) in &state.data.missiles {
            self.draw_missile(context, missile, state)?;
        }

        for (_, explosion) in &state.data.explosions {
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
        )?;

        Ok(())
    }

    fn draw_ship(&self, context: &mut Context, ship: &Ship, state: &State)
        -> GameResult<bool>
    {
        let craft = get!(state.data.crafts, ship.craft);
        let body  = get!(state.data.bodies, craft.body);
        let pos   = get!(state.data.positions, body.pos);

        graphics::draw(
            context,
            &self.ship,
            DrawParam::new()
                .dest(pos.0)
                .rotation(Vec2::unit_x().angle(body.dir).0)
                .scale([30.0, 30.0])
                .color(
                    [ship.color[0], ship.color[1], ship.color[2], 1.0]
                        .into(),
                ),
        )?;

        Ok(true)
    }

    fn draw_missile(&self,
        context: &mut Context,
        missile: &Missile,
        state:   &State,
    )
        -> GameResult<bool>
    {
        let craft  = get!(state.data.crafts, missile.craft);
        let target = get!(state.data.targets, missile.target);
        let body   = get!(state.data.bodies, craft.body);
        let pos    = get!(state.data.positions, body.pos);

        graphics::draw(
            context,
            &self.missile,
            DrawParam::new()
                .dest(pos.0)
                .scale([4.0, 4.0])
        )?;

        let line = Mesh::new_line(
            context,
            &[pos.0, target.value],
            1.5,
            [0.0, 1.0, 0.0, 1.0].into(),
        )?;

        graphics::draw(
            context,
            &line,
            DrawParam::new(),
        )?;

        Ok(true)
    }

    fn draw_explosion(&self,
        context:   &mut Context,
        explosion: &Explosion,
        state:     &State,
    )
        -> GameResult<bool>
    {
        let body = get!(state.data.bodies, explosion.body);
        let pos  = get!(state.data.positions, body.pos);

        let alpha = explosion.strength_left / explosion.strength_total;
        let size  = explosion.strength_total * 2.0;

        graphics::draw(
            context,
            &self.explosion,
            DrawParam::new()
                .dest(pos.0)
                .scale([size, size])
                .color([1.0, 1.0, 1.0, alpha].into())
        )?;

        Ok(true)
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
Fuels: {}/{}
Guidances: {}/-
Healths: {}/{}
Players: {}/-
Missiles: {}/{}
Positions: {}/{}
Ships: {}/{}
Targets: {}/{}
Velocities: {}/{}
---
Updates per s: {}
Removals per s: {}",
                diagnostics.num_bodies, state.data.bodies.len(),
                diagnostics.num_crafts, state.data.crafts.len(),
                diagnostics.num_explosions, state.data.explosions.len(),
                diagnostics.num_fuels, state.data.fuels.len(),
                diagnostics.num_guidances,
                diagnostics.num_healths, state.data.healths.len(),
                diagnostics.num_players,
                diagnostics.num_missiles, state.data.missiles.len(),
                diagnostics.num_positions, state.data.positions.len(),
                diagnostics.num_ships, state.data.ships.len(),
                diagnostics.num_targets, state.data.targets.len(),
                diagnostics.num_velocities, state.data.velocities.len(),
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

        let mut input_events = String::from("Input:\n");
        for event in &input.events.inner {
            input_events.push_str(&format!("{:?}\n", event));
        }

        graphics::draw(
            context,
            &Text::new(input_events),
            DrawParam::new()
                .dest([20.0, 520.0])
        )?;

        for ship in state.data.ships.values() {
            if self.draw_ship_status(context, ship, state)? {
                // There should only be one ship owned by the local player, so
                // let's quit.
                break;
            }
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
        -> GameResult<bool>
    {
        let craft  = get!(state.data.crafts, ship.craft);
        let fuel   = get!(state.data.fuels, craft.fuel);
        let health = get!(state.data.healths, craft.health);

        if state.own_id != Some(craft.owner) {
            return Ok(false);
        }

        let (width, _) = graphics::drawable_size(context);

        let status = format!("Ship Status
Structural Integrity: {:.2}
Fuel: {:.2}
Heavy Missiles: {}",
            health.value,
            fuel.0,
            ship.missiles,
        );

        graphics::draw(
            context,
            &Text::new(status),
            DrawParam::new()
                .dest([width - 200.0, 20.0])
        )?;

        Ok(true)
    }
}
