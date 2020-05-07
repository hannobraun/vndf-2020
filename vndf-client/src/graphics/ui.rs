use crate::{
    game::Game,
    graphics::{
        self,
        elements::ScreenElement,
        screen::Screen,
        transforms::NativeTransform,
    },
};


pub fn elements(game: &Game, screen: &Screen) -> Vec<Element> {
    let mut elements = Vec::new();

    elements.push(Element::instructions(game, screen));
    elements.push(Element::zoom(game, screen));

    if let Some(element) = Element::frame_time(game, screen) {
        elements.push(element);
    }
    if let Some(element) = Element::diagnostics(game, screen) {
        elements.push(element);
    }
    if let Some(element) = Element::input_events(game, screen) {
        elements.push(element);
    }

    elements
}


pub struct Element {
    pub text:      String,
    pub transform: NativeTransform,
}

impl Element {
    pub fn instructions(game: &Game, screen: &Screen) -> Self {
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

        let pos = graphics::Pnt2::new(20.0, 20.0);

        Self::new(text, pos, screen)
    }

    pub fn zoom(game: &Game, screen: &Screen) -> Self {
        let text = format!("Zoom: {:.3}x", game.input.zoom);

        let pos = graphics::Pnt2::new(20.0, 150.0);

        Self::new(text, pos, screen)
    }

    pub fn frame_time(game: &Game, screen: &Screen) -> Option<Self> {
        if !game.input.config.diagnostics.frame_time {
            return None;
        }

        let report = game.state.frame_time.report();
        let text = format!(
            "Frame time:\n{} ms (avg {}/{}/{})",
            report.latest.whole_milliseconds(),
            report.avg_1.whole_milliseconds(),
            report.avg_2.whole_milliseconds(),
            report.avg_3.whole_milliseconds(),
        );

        let pos = graphics::Pnt2::new(20.0, 180.0);

        Some(
            Self::new(text, pos, screen)
        )
    }

    pub fn diagnostics(game: &Game, screen: &Screen) -> Option<Self> {
        if !game.input.config.diagnostics.components {
            return None;
        }

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

            let pos = graphics::Pnt2::new(20.0, 220.0);

            Self::new(text, pos, screen)
        })
    }

    pub fn input_events(game: &Game, screen: &Screen) -> Option<Self> {
        if !game.input.config.diagnostics.input {
            return None;
        }

        let mut text = String::from("Input:\n");
        for event in game.events.iter().rev() {
            text.push_str(&format!("{}\n", event));
        }

        let pos = graphics::Pnt2::new(20.0, 520.0);

        Some(
            Self::new(text, pos, screen)
        )
    }

    pub fn new(text: String, pos: graphics::Pnt2, screen: &Screen) -> Self {
        let transform = ScreenElement { pos, .. ScreenElement::default() }
            .transform(screen.size)
            .to_native();

        Self {
            text,
            transform,
        }
    }
}
