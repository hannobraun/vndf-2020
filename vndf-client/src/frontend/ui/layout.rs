use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::elements::{
            Element as _,
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

    pub fn draw_legacy_element(&mut self, element: &'r ui::Element) {
        let text_panel = TextPanel::new(self.res, &element.text, self.next_pos)
            .unwrap();

        self.next_pos.y += text_panel.size().height + self.margin;

        text_panel.draw(self.res, self.frame)
    }
}
