use wgpu_glyph::{
    GlyphBrush,
    GlyphBrushBuilder,
    GlyphCruncher as _,
    Scale,
    Section,
};
use winit::event::Event;

use crate::{
    frontend::{
        drawables::Drawables,
        drawers::Frame,
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


pub struct Basic {
    glyph_brush: GlyphBrush<'static, ()>,
}

impl Basic {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat)
        -> Result<Self, wgpu_glyph::rusttype::Error>
    {
        let font = include_bytes!("fonts/Tuffy_Bold.ttf");
        let glyph_brush = GlyphBrushBuilder::using_font_bytes(&font[..])?
            .build(&device, texture_format);

        Ok(
            Self {
                glyph_brush,
            }
        )
    }
}

impl super::Ui for Basic {
    fn draw(&mut self,
        device:    &wgpu::Device,
        frame:     &mut Frame,
        drawables: &mut Drawables,
        game:      &Game,
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

            let size = match self.glyph_brush.glyph_bounds(section) {
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

            drawables.panel.draw(
                device,
                frame,
                vert::simple::Uniforms {
                    transform: element.transform(frame.screen.size).into(),
                },
                frag::simple::Uniforms {
                    color: [0.0, 0.0, 0.0, 0.95].into(),
                },
            );

            self.glyph_brush.queue(section);
        }

        self.glyph_brush
            .draw_queued(
                device,
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
