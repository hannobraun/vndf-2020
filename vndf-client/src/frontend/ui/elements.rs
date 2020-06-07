mod text;


pub use self::text::Text;


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


pub fn draw(
    res:   &mut DrawResources,
    frame: &mut Frame,
    pos:   graphics::Pnt2,
    text:  &str,
)
    -> graphics::Size
{
    let text = Text::new(text, pos);

    let text_size = match text.size(res) {
        Some(size) => size,
        None       => panic!("Tried rendering text without size"),
    };

    const PADDING: graphics::Scalar = 3.0;
    let padding = graphics::Size::new(
        PADDING * 2.0,
        PADDING * 2.0,
    );

    let panel_size = text_size + padding;

    panel(
        res,
        frame,
        pos + text_size / 2.0,
        panel_size,
    );

    text.draw(res, frame);

    panel_size
}

pub fn panel(
    res:   &mut DrawResources,
    frame: &mut Frame,
    pos:   graphics::Pnt2,
    size:  graphics::Size,
) {
    let element = ScreenElement {
        size,
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
