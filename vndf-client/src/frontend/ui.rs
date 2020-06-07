mod element;


use std::iter;

use wgpu_glyph::Section;

use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
    },
    game::Game,
    graphics,
    ui,
};

use self::element::draw;


pub struct Ui;

impl Ui {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&mut self,
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
            draw(
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
        let size = draw(
            res,
            frame,
            next_pos,
            element.text.as_str(),
            sections,
        );

        next_pos.y += size.height + MARGIN;
    }
}
