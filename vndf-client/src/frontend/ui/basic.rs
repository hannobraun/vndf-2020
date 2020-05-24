use wgpu_glyph::{
    GlyphCruncher as _,
    Scale,
    Section,
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
        let scale  = Scale::uniform(16.0 * frame.screen.scale_factor);

        for element in ui::Elements::new(game, &frame.screen).elements() {
            let text  = element.text.as_str();
            let pos   = element.pos * frame.screen.scale_factor;
            let color = [1.0, 1.0, 1.0, 1.0];

            let section = Section {
                text,
                screen_position: (pos.x, pos.y),
                scale,
                color,
                .. Section::default()
            };

            let size = res.drawables.text.glyph_brush.glyph_bounds(section);
            let size = match size {
                Some(size) => size,
                None       => continue,
            };
            let size = graphics::Size::new(size.width(), size.height())
                / frame.screen.scale_factor;

            const MARGIN: f32 = 5.0;
            let margin = graphics::Size::new(MARGIN * 2.0, MARGIN * 2.0);

            let element = ScreenElement {
                size:  size + margin,
                pos:   pos + size * frame.screen.scale_factor / 2.0,
                angle: graphics::Angle::zero(),
            };

            res.drawables.panel.draw(
                &res.device,
                frame,
                vert::simple::Uniforms {
                    transform: element.transform(frame.screen.size).into(),
                },
                frag::simple::Uniforms {
                    color: [0.0, 0.0, 0.0, 0.95].into(),
                },
            );

            res.drawables.text.glyph_brush.queue(section);
        }

        res.drawables.text.glyph_brush
            .draw_queued(
                &res.device,
                &mut frame.encoder,
                &frame.output.view,
                frame.screen.size.width as u32,
                frame.screen.size.height as u32,
            )
            // I've checked the code, and it doesn't look like this
            // actually returns any errors.
            .unwrap();

        Ok(())
    }

    fn handle_event(&mut self, _: &Event<()>, _: &Screen) {}
}
