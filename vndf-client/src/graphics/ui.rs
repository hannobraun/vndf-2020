use crate::game::Game;


pub struct Ui {
    pub instructions: String,
    pub zoom:         String,
    pub frame_time:   String,
    pub diagnostics:  Option<String>,
    pub input_events: String,
}

impl Ui {
    pub fn new(game: &Game) -> Self {
        let instructions = format!(
            "Instructions:\n\
            Turn left - {}\n\
            Turn right - {}\n\
            Thrust On - {}\n\
            Thrust Off - {}\n\
            Zoom Camera - Mouse Wheel\n\
            End game - {}",
            game.input.config.input.left,
            game.input.config.input.right,
            game.input.config.input.thrust_on,
            game.input.config.input.thrust_off,
            game.input.config.input.quit,
        );

        let zoom = format!("Zoom: {:.3}x", game.input.zoom);

        let report = game.state.frame_time.report();
        let frame_time = format!(
            "Frame time:\n{} ms (avg {}/{}/{})",
            report.latest.whole_milliseconds(),
            report.avg_1.whole_milliseconds(),
            report.avg_2.whole_milliseconds(),
            report.avg_3.whole_milliseconds(),
        );

        let diagnostics = game.state.diagnostics.map(|diagnostics| {
            format!(
                "Components:\n\
                Bodies: {}/{}\n\
                Crafts: {}/{}\n\
                Explosions: {}/{}\n\
                Fuels: {}/{}\n\
                Healths: {}/{}\n\
                Planets: {}/{}\n\
                Players: {}/-\n\
                Positions: {}/{}\n\
                Ships: {}/{}\n\
                Velocities: {}/{}\n\
                ---\n\
                Updates per s: {}\n\
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
            )
        });

        let mut input_events = String::from("Input:\n");
        for event in game.events.iter().rev() {
            input_events.push_str(&format!("{}\n", event));
        }

        Self {
            instructions,
            zoom,
            frame_time,
            diagnostics,
            input_events,
        }
    }
}
