use wgpu_glyph::{
    Section,
    Text,
};
use winit::event::Event;

use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        shaders::{
            frag,
            vert,
        },
    },
    game::Game,
    graphics::{
        self,
        elements::ScreenElement,
        screen::Screen,
    },
    ui,
};


pub struct Basic;

impl Basic {
    pub fn new() -> Self {
        Self
    }
}

impl super::Ui for Basic {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        game:  &Game,
    )
        -> Result<(), ()>
    {
        let elements = ui::Elements::new(game, &frame.screen);
        let sections: Vec<_> = elements
            .elements()
            .into_iter()
            .filter_map(|element| {
                let text = vec![
                    Text::default()
                        .with_text(element.text.as_str())
                        .with_scale(16.0 * frame.screen.scale_factor)
                        .with_color([1.0, 1.0, 1.0, 1.0]),
                ];

                let pos = element.pos * frame.screen.scale_factor;

                let section = Section {
                    text,
                    screen_position: (pos.x, pos.y),
                    .. Section::default()
                };

                let size = match res.drawables.text.bounds(&section) {
                    Some(size) => size / frame.screen.scale_factor,
                    None       => return None,
                };

                const MARGIN: f32 = 5.0;
                let margin = graphics::Size::new(MARGIN * 2.0, MARGIN * 2.0);

                let element = ScreenElement {
                    size:  size + margin,
                    pos:   pos + size * frame.screen.scale_factor / 2.0,
                    angle: graphics::Angle::zero(),
                };

                res.drawables.square.draw(
                    &res.device,
                    frame,
                    vert::simple::Uniforms {
                        transform: element.transform(frame.screen.size).into(),
                    },
                    frag::simple::Uniforms {
                        color: [0.0, 0.0, 0.0, 0.95].into(),
                    },
                );

                Some(section)
            })
            .collect();

        res.drawables.text.draw(
            &res.device,
            frame,
            sections,
        );

        Ok(())
    }

    fn handle_event(&mut self, _: &Event<()>, _: &Screen) {}
}
