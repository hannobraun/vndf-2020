use std::io;

use winit::event::{
    Event,
    WindowEvent,
};

use crate::{
    game::Game,
    graphics::{
        self,
        elements::{
            UiElement,
            WorldElement,
        },
    },
    shared::world::behavior::{
        planets::Planet,
        ships::Ship,
    },
};

use super::{
    drawables::Drawables,
    meshes::{
        self,
        Meshes,
    },
    window::Window,
};


pub struct Renderer {
    pub surface:               wgpu::Surface,
    pub device:                wgpu::Device,
    pub queue:                 wgpu::Queue,
    pub swap_chain_descriptor: wgpu::SwapChainDescriptor,
    pub swap_chain:            wgpu::SwapChain,

    drawables: Drawables,
}

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self, Error> {
        let surface = wgpu::Surface::create(&window.0);

        let adapter =
            wgpu::Adapter::request(
                &wgpu::RequestAdapterOptions {
                    power_preference:   wgpu::PowerPreference::Default,
                    compatible_surface: Some(&surface),
                },
                wgpu::BackendBit::all(),
            )
            .await
            .ok_or(Error::AdapterRequest)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    extensions: wgpu::Extensions {
                        anisotropic_filtering: false,
                    },
                    limits: wgpu::Limits::default(),
                },
            )
            .await;

        let meshes = Meshes::new()
            .map_err(|err| Error::Meshes(err))?;
        let drawables = Drawables::new(&device, &meshes)?;

        let size = window.0.inner_size();

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage:        wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format:       wgpu::TextureFormat::Bgra8UnormSrgb,
            width:        size.width,
            height:       size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(
            &surface,
            &swap_chain_descriptor,
        );

        Ok(
            Self {
                surface,
                device,
                queue,
                swap_chain_descriptor,
                swap_chain,

                drawables,
            }
        )
    }

    pub fn handle_event(&mut self, event: &Event<()>, game: &Game)
        -> Result<(), wgpu::TimeOut>
    {
        match event {
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                self.swap_chain_descriptor.width  = size.width;
                self.swap_chain_descriptor.height = size.height;

                self.swap_chain = self.device.create_swap_chain(
                    &self.surface,
                    &self.swap_chain_descriptor,
                );
            }
            Event::RedrawRequested(_) => {
                let frame = self.swap_chain.get_next_texture()?;

                let mut encoder = self.device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor { label: None }
                );

                self.draw_background(&frame, &mut encoder);

                for planet in game.state.data.planets.values() {
                    self.draw_planet(&frame, &mut encoder, planet, game);
                }
                for ship in game.state.data.ships.values() {
                    self.draw_ship(&frame, &mut encoder, ship, game);
                }

                self.queue.submit(&[encoder.finish()]);
            }
            _ => {}
        }

        Ok(())
    }

    fn draw_background(&self,
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

    fn draw_planet(&self,
        frame:   &wgpu::SwapChainOutput,
        encoder: &mut wgpu::CommandEncoder,
        planet:  &Planet,
        game:    &Game,
    ) {
        let transform = WorldElement::from(planet)
            .transform(&game.state.camera, self.screen_size());

        self.drawables.planet.draw(
            &self.device,
            frame,
            encoder,
            transform,
        );
    }

    fn draw_ship(&self,
        frame:   &wgpu::SwapChainOutput,
        encoder: &mut wgpu::CommandEncoder,
        ship:    &Ship,
        game:    &Game,
    )
        -> Option<()>
    {
        let transform = UiElement::from_ship(ship, game, self.screen_size())?
            .transform(self.screen_size());

        self.drawables.ship.draw(
            &self.device,
            frame,
            encoder,
            transform,
        );

        Some(())
    }

    fn screen_size(&self) -> graphics::Size {
        graphics::Size::new(
            self.swap_chain_descriptor.width  as f32,
            self.swap_chain_descriptor.height as f32,
        )
    }
}


#[derive(Debug)]
pub enum Error {
    AdapterRequest,
    Io(io::Error),
    Meshes(meshes::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
