use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::elements::{
            Element,
            TextPanel,
        }
    },
    graphics,
    ui,
};


pub struct Layout {
    next_pos: graphics::Pnt2,
    margin:   f32,
}

impl Layout {
    pub fn new(
        initial_pos: graphics::Pnt2,
        margin:      f32,
    ) -> Self {
        Self {
            next_pos: initial_pos,
            margin,
        }
    }

    pub fn draw(&mut self,
        res:     &mut DrawResources,
        frame:   &mut Frame,
        element: impl Element,
    ) {
        self.draw_iter(res, frame, Some(element))
    }

    pub fn draw_iter(&mut self,
        res:      &mut DrawResources,
        frame:    &mut Frame,
        elements: impl IntoIterator<Item=impl Element>,
    ) {
        for element in elements {
            let offset_y = element.size().height + self.margin;
            element.draw(res, frame, self.next_pos);
            self.next_pos.y += offset_y;
        }
    }

    pub fn draw_legacy_element(&mut self,
        res:     &mut DrawResources,
        frame:   &mut Frame,
        element: &ui::Element,
    ) {
        let text_panel = TextPanel::new(res, &element.text)
            .unwrap();

        self.draw(res, frame, text_panel);
    }
}
