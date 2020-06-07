use std::iter;

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
        let mut sections = Vec::new();
        let     elements = ui::Elements::new(game, &frame.screen);

        layout_panels(
            res,
            frame,
            iter::once(&elements.instructions)
                .chain(elements.frame_time.as_ref())
                .chain(elements.diagnostics.as_ref())
                .chain(elements.input_events.as_ref()),
            &mut sections,
        );

        let other_elements = elements.own_ship_status.iter()
            .chain(&elements.ship_info)
            .chain(&elements.orbit_info);

        for element in other_elements {
            draw_panel(
                res,
                frame,
                element.pos,
                element.text.as_str(),
                &mut sections,
            );
        }

        res.drawables.text.draw(
            &res.device,
            frame,
            sections,
        );

        Ok(())
    }

    fn handle_event(&mut self, _: &Event<()>, _: &Screen) {}
}


fn layout_panels<'r>(
    res:      &mut DrawResources,
    frame:    &mut Frame,
    elements: impl Iterator<Item=&'r ui::Element>,
    sections: &mut Vec<Section<'r>>,
) {
    const MARGIN: f32 = 20.0;

    let mut next_pos = graphics::Pnt2::new(MARGIN, MARGIN);

    for element in elements {
        let size = draw_panel(
            res,
            frame,
            next_pos,
            element.text.as_str(),
            sections,
        );

        next_pos.y += size.height + MARGIN;
    }
}

fn draw_panel<'r>(
    res:      &mut DrawResources,
    frame:    &mut Frame,
    pos:      graphics::Pnt2,
    text:     &'r str,
    sections: &mut Vec<Section<'r>>,
)
    -> graphics::Size
{
    let text = vec![
        Text::default()
            .with_text(text)
            .with_scale(16.0)
            .with_color([1.0, 1.0, 1.0, 1.0]),
    ];

    let section = Section {
        text,
        screen_position: (pos.x, pos.y),
        .. Section::default()
    };

    let size = match res.drawables.text.bounds(&section) {
        Some(size) => size,
        None       => panic!("Tried rendering text without size"),
    };

    const PADDING: graphics::Scalar = 3.0;
    let padding = graphics::Size::new(
        PADDING * 2.0,
        PADDING * 2.0,
    );

    let panel_size = size + padding;

    let element = ScreenElement {
        size:  panel_size,
        pos:   pos + size / 2.0,
        angle: graphics::Angle::zero(),
    };
    let transform = element
        .transform(&frame.screen)
        .into();

    res.drawables.square.draw(
        &res.device,
        frame,
        vert::simple::Uniforms {
            transform,
        },
        frag::simple::Uniforms {
            color: [0.0, 0.0, 0.0, 0.95].into(),
        },
    );

    sections.push(section);

    panel_size
}
