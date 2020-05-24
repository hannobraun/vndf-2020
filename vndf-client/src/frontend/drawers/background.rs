use crate::graphics;

use super::Frame;


pub fn draw_background(res: &mut Frame) {
    res.encoder.begin_render_pass(
        &wgpu::RenderPassDescriptor {
            color_attachments: &[
                wgpu::RenderPassColorAttachmentDescriptor {
                    attachment:     &res.output.view,
                    resolve_target: None,
                    load_op:        wgpu::LoadOp::Clear,
                    store_op:       wgpu::StoreOp::Store,
                    clear_color:    graphics::BACKGROUND_COLOR,
                },
            ],
            depth_stencil_attachment: None,
        },
    );
}
