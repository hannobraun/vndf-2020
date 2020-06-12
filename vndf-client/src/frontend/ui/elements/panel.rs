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
    pub size: graphics::Size,
}

impl Element for Panel {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        let element = ScreenElement {
            size: self.size,
            pos,
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
