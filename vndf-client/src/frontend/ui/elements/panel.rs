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
    graphics::{
        self,
        elements::ScreenElement,
    },
};

use super::Element;


pub struct Panel {
    pub pos:  graphics::Pnt2,
    pub size: graphics::Size,
}

impl Element for Panel {
    fn size(&self, _: &mut DrawResources) -> Option<graphics::Size> {
        Some(self.size)
    }

    fn draw(self, res: &mut DrawResources, frame: &mut Frame) {
        let element = ScreenElement {
            size: self.size,
            pos:  self.pos,
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
    }
}
