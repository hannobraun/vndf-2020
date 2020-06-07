use wgpu_glyph::{
    Section,
    Text,
};

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


pub fn draw<'r>(
    res:      &mut DrawResources,
    frame:    &mut Frame,
    pos:      graphics::Pnt2,
    text:     &'r str,
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

    let text_size = match res.drawables.text.bounds(&section) {
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

    res.drawables.text.draw(
        &res.device,
        frame,
        Some(section),
    );

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
