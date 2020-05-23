use crate::graphics;


pub struct Background;

impl Background {
    pub fn draw(
        frame:   &wgpu::SwapChainOutput,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment:     &frame.view,
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
}
