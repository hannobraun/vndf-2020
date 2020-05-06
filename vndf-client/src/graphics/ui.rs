use crate::game::Game;


pub struct Ui {
    pub instructions: Element,
    pub zoom:         Element,
    pub frame_time:   Element,
    pub diagnostics:  Option<Element>,
    pub input_events: Element,
}

impl Ui {
    pub fn new(game: &Game) -> Self {
        Self {
            instructions: Element::instructions(game),
            zoom:         Element::zoom(game),
            frame_time:   Element::frame_time(game),
            diagnostics:  Element::diagnostics(game),
            input_events: Element::input_events(game),
        }
    }
}


pub struct Element {
    pub text: String,
}

impl Element {
    pub fn instructions(game: &Game) -> Self {
        let text = format!(
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

        Self::new(text)
    }

    pub fn zoom(game: &Game) -> Self {
        let text = format!("Zoom: {:.3}x", game.input.zoom);

        Self::new(text)
    }

    pub fn frame_time(game: &Game) -> Self {
        let report = game.state.frame_time.report();
        let text = format!(
            "Frame time:\n{} ms (avg {}/{}/{})",
            report.latest.whole_milliseconds(),
            report.avg_1.whole_milliseconds(),
            report.avg_2.whole_milliseconds(),
            report.avg_3.whole_milliseconds(),
        );

        Self::new(text)
    }

    pub fn diagnostics(game: &Game) -> Option<Self> {
        game.state.diagnostics.map(|diagnostics| {
            let text = format!(
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
            );

            Self::new(text)
        })
    }

    pub fn input_events(game: &Game) -> Self {
        let mut text = String::from("Input:\n");
        for event in game.events.iter().rev() {
            text.push_str(&format!("{}\n", event));
        }

        Self::new(text)
    }

    pub fn new(text: String) -> Self {
        Self {
            text,
        }
    }
}
