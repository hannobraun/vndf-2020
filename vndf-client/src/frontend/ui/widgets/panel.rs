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
        ui::widgets::{
            DrawAt,
            Widget,
        },
    },
    graphics::{
        self,
        elements::ScreenElement,
    },
};


pub struct Panel {
    pub size: graphics::Size,
}

impl Widget for Panel {
    fn size(&self) -> graphics::Size {
        self.size
    }
}

impl DrawAt for Panel {
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
