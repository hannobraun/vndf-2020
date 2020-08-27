use crate::graphics;

use super::Frame;


pub fn draw_background(frame: &mut Frame) {
    frame.encoder.begin_render_pass(
        &wgpu::RenderPassDescriptor {
            color_attachments: &[
                wgpu::RenderPassColorAttachmentDescriptor {
                    attachment:     &frame.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(graphics::BACKGROUND_COLOR),
                        store: true,
                    },
                },
            ],
            depth_stencil_attachment: None,
        },
    );
}
