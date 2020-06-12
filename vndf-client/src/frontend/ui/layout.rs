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


pub struct Layout<'r> {
    res:      &'r mut DrawResources,
    frame:    &'r mut Frame,
    next_pos: graphics::Pnt2,
    margin:   f32,
}

impl<'r> Layout<'r> {
    pub fn new(
        res:         &'r mut DrawResources,
        frame:       &'r mut Frame,
        initial_pos: graphics::Pnt2,
        margin:      f32,
    ) -> Self {
        Self {
            res,
            frame,
            next_pos: initial_pos,
            margin,
        }
    }

    pub fn draw(&mut self, element: impl Element) {
        let offset_y = element.size().height + self.margin;
        element.draw(self.res, self.frame, self.next_pos);
        self.next_pos.y += offset_y;
    }

    pub fn draw_legacy_element(&mut self, element: &'r ui::Element) {
        let text_panel = TextPanel::new(self.res, &element.text)
            .unwrap();

        self.draw(text_panel);
    }
}
