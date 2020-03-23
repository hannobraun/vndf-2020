use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
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
    game::{
        input::Input,
        state::State,
    },
    shared::{
        game::{
            explosions::Explosion,
            loot::Loot,
            missiles::Missile,
            physics::Body,
            planets::{
                Planet,
                Planets,
            },
            ships::Ship,
        },
        math::{
            prelude::*,
            Pnt2,
            Vec2,
        }
    },
    transforms::{
        Screen,
        ScreenTransform,
        WorldTransform,
    },
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
            &[
                [ 0.6,  0.0],
                [-0.4,  0.4],
                [-0.1,  0.0],
                [-0.4, -0.4],
                ],
                [1.0, 1.0, 1.0, 1.0].into(),
            )?;
        let square = Mesh::new_polygon(
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
                circle,
                ship,
                square,
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

    fn draw_world(&self,
        context: &mut Context,
        state:   &State,
    )
        -> GameResult
    {
        for planet in state.data.planets.values() {
            self.draw_planet(context, planet, state)?;
        }
        for loot in state.data.loots.values() {
            self.draw_loot(context, loot, state)?;
        }
        for ship in state.data.ships.values() {
            self.draw_ship(context, ship, state)?;
        }
        for missile in state.data.missiles.values() {
            self.draw_missile(context, missile, state)?;
        }
        for explosion in state.data.explosions.values() {
            self.draw_explosion(context, explosion, state)?;
        }

        Ok(())
    }

    fn draw_planet(&self, context: &mut Context, planet: &Planet, state: &State)
        -> GameResult
    {
        draw(
            context,
            &WorldTransform(&state.camera),
            &self.circle,
            DrawParam::world()
                .dest(planet.pos)
                .scale(Vec2::new(planet.size, planet.size))
        )?;

        Ok(())
    }

    fn draw_ship(&self, context: &mut Context, ship: &Ship, state: &State)
        -> GameResult<bool>
    {
        let craft = get!(state.data.crafts, &ship.craft);
        let body  = get!(state.data.bodies, &craft.body);
        let pos   = get!(state.data.positions, &body.pos);

        self.draw_projected_path(context, &craft.body, ship.color, state)?;

        let pos = state.camera.world_to_screen(context, pos);

        draw(
            context,
            &ScreenTransform,
            &self.ship,
            DrawParam::screen()
                .dest(pos)
                .rotation(Vec2::unit_x().angle(body.dir).0)
                .scale(Vec2::new(30.0, 30.0))
                .color([ship.color[0], ship.color[1], ship.color[2], 1.0]),
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
        let craft  = get!(state.data.crafts, &missile.craft);
        let target = get!(state.data.targets, &missile.target);
        let body   = get!(state.data.bodies, &craft.body);
        let pos    = get!(state.data.positions, &body.pos);

        let pos    = state.camera.world_to_screen(context, pos);
        let target = state.camera.world_to_screen(context, target.value);

        draw(
            context,
            &ScreenTransform,
            &self.square,
            DrawParam::screen()
                .dest(pos)
                .scale(Vec2::new(4.0, 4.0))
        )?;

        let line = Mesh::new_line(
            context,
            &[pos.0, target.0],
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

    fn draw_projected_path(&self,
        context: &mut Context,
        body:    impl Into<handle::Weak<Body>>,
        color:   [f32; 3],
        state:   &State,
    )
        -> GameResult<bool>
    {
        let mut body = get!(state.data.bodies, body).clone();
        body.acc = Vec2::zero();

        let pos = *get!(state.data.positions,  &body.pos);
        let vel = *get!(state.data.velocities, &body.vel);

        let mut positions  = OneStore { handle: (&body.pos).into(), data: pos };
        let mut velocities = OneStore { handle: (&body.vel).into(), data: vel };

        let mut previous = pos;

        for _ in 0 .. 100 {
            body.update(
                1.0,
                Planets(&state.data.planets),
                &mut positions,
                &mut velocities,
            );

            let current = positions.data;

            if previous == current {
                break;
            }

            let previous_s = state.camera.world_to_screen(context, previous);
            let current_s  = state.camera.world_to_screen(context, current);

            let line = Mesh::new_line(
                context,
                &[previous_s.0, current_s.0],
                1.5,
                [color[0], color[1], color[2], 0.5].into(),
            )?;
            draw(
                context,
                &ScreenTransform,
                &line,
                DrawParam::screen(),
            )?;

            previous = current;
        }

        Ok(true)
    }

    fn draw_explosion(&self,
        context:   &mut Context,
        explosion: &Explosion,
        state:     &State,
    )
        -> GameResult<bool>
    {
        let pos = get!(state.data.positions, &explosion.pos);

        let alpha = explosion.strength_left / explosion.strength_total;
        let size  = explosion.strength_total * 2.0;

        let pos = state.camera.world_to_screen(context, pos);

        draw(
            context,
            &ScreenTransform,
            &self.circle,
            DrawParam::screen()
                .dest(pos)
                .scale(Vec2::new(size, size))
                .color([1.0, 1.0, 1.0, alpha])
        )?;

        Ok(true)
    }

    pub fn draw_loot(&self,
        context: &mut Context,
        loot:    &Loot,
        state:   &State,
    )
        -> GameResult<bool>
    {
        let size = 10.0;

        let body = get!(state.data.bodies,    &loot.body);
        let pos  = get!(state.data.positions, &body.pos);

        draw(
            context,
            &WorldTransform(&state.camera),
            &self.square,
            DrawParam::world()
                .dest(pos)
                .scale(Vec2::new(size, size))
                .color([1.0, 1.0, 1.0, 1.0])
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
        let instructions = format!(
"Instructions:
Turn left - {}
Turn right - {}
Accelerate - {}
Shoot - {} (aim with mouse)
Zoom Camera - Mouse Wheel
End game - Escape",
            input.config.input.left,
            input.config.input.right,
            input.config.input.thrust,
            input.config.input.launch,
        );

        draw(
            context,
            &ScreenTransform,
            &Text::new(instructions),
            DrawParam::screen()
                .dest(Screen(Pnt2::new(20.0, 20.0)))
        )?;

        draw(
            context,
            &ScreenTransform,
            &Text::new(format!("Zoom: {:.3}x", input.zoom)),
            DrawParam::screen()
                .dest(Screen(Pnt2::new(20.0, 150.0))),
        )?;

        if input.config.diagnostics.frame_time {
            let report = state.frame_time.report();
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
                    .dest(Screen(Pnt2::new(20.0, 180.0)))
            )?;
        }

        if input.config.diagnostics.components {
            if let Some(diagnostics) = state.diagnostics {
                let diagnostics = format!(
"Components:
Bodies: {}/{}
Crafts: {}/{}
Explosions: {}/{}
Fuels: {}/{}
Guidances: {}/-
Healths: {}/{}
Loots: {}/{}
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
                    diagnostics.bodies, state.data.bodies.len(),
                    diagnostics.crafts, state.data.crafts.len(),
                    diagnostics.explosions, state.data.explosions.len(),
                    diagnostics.fuels, state.data.fuels.len(),
                    diagnostics.guidances,
                    diagnostics.healths, state.data.healths.len(),
                    diagnostics.loots, state.data.loots.len(),
                    diagnostics.planets, state.data.planets.len(),
                    diagnostics.players,
                    diagnostics.missiles, state.data.missiles.len(),
                    diagnostics.positions, state.data.positions.len(),
                    diagnostics.ships, state.data.ships.len(),
                    diagnostics.targets, state.data.targets.len(),
                    diagnostics.velocities, state.data.velocities.len(),
                    state.statistics.updates.len(),
                    state.statistics.removals.len(),
                );

                draw(
                    context,
                    &ScreenTransform,
                    &Text::new(diagnostics),
                    DrawParam::screen()
                        .dest(Screen(Pnt2::new(20.0, 220.0)))
                )?;
            }
        }

        if input.config.diagnostics.input {
            let mut input_events = String::from("Input:\n");
            for event in input.events.iter().rev() {
                input_events.push_str(&format!("{}\n", event));
            }

            draw(
                context,
                &ScreenTransform,
                &Text::new(input_events),
                DrawParam::screen()
                    .dest(Screen(Pnt2::new(20.0, 520.0)))
            )?;
        }

        for ship in state.data.ships.values() {
            if self.draw_ship_status(context, ship, state)? {
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
                .dest(input.pointer_screen)
                .scale(Vec2::new(10.0, 10.0))
        )?;

        mouse::set_cursor_hidden(context, true);

        Ok(())
    }

    fn draw_ship_status(&self,
        context: &mut Context,
        ship:    &Ship,
        state:   &State,
    )
        -> GameResult<bool>
    {
        let craft  = get!(state.data.crafts, &ship.craft);
        let fuel   = get!(state.data.fuels, &craft.fuel);
        let health = get!(state.data.healths, &craft.health);

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

        draw(
            context,
            &ScreenTransform,
            &Text::new(status),
            DrawParam::screen()
                .dest(Screen(Pnt2::new(width - 200.0, 20.0)))
        )?;

        Ok(true)
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
