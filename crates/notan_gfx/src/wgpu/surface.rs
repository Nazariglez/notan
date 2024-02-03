use super::context::Context;
use crate::attrs::GfxAttributes;
use crate::Texture;
use notan_core::window::NotanWindow;
use std::sync::Arc;
use wgpu::{
    Device, Surface as RawSurface, SurfaceCapabilities, SurfaceConfiguration, SurfaceTexture,
};

#[derive(Clone)]
pub(crate) struct Surface {
    pub surface: Arc<RawSurface<'static>>,
    pub config: SurfaceConfiguration,
    pub capabilities: Arc<SurfaceCapabilities>,
    pub depth_texture: Texture,
}

impl Surface {
    pub fn new<W: NotanWindow>(
        ctx: &mut Context,
        window: &W,
        attrs: GfxAttributes,
        depth_texture: Texture,
    ) -> Result<Self, String> {
        log::trace!("Creating a new Surface for Window {:?}", window.id());
        let surface = unsafe { ctx.instance.create_surface(window) }.map_err(|e| e.to_string())?;

        if !ctx.is_surface_compatible(&surface) {
            log::trace!(
                "Generating WGPU adapter compatible with {:?} surface.",
                window.id()
            );
            ctx.ensure_surface_compatibility(&surface)?;
        }

        let (width, height) = window.physical_size();
        let capabilities = surface.get_capabilities(&ctx.adapter);
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: capabilities.formats[0],
            width,
            height,
            present_mode: if attrs.vsync {
                wgpu::PresentMode::AutoVsync
            } else {
                wgpu::PresentMode::AutoNoVsync
            },
            desired_maximum_frame_latency: 2,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&ctx.device, &config);

        println!(
            "Surface size({:?} {:?}) depth_texture({:?})",
            config.width, config.height, depth_texture.size
        );

        Ok(Self {
            surface: Arc::new(surface),
            config,
            capabilities: Arc::new(capabilities),
            depth_texture,
        })
    }

    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(device, &self.config);
    }

    pub fn frame(&self) -> Result<SurfaceTexture, String> {
        self.surface
            .get_current_texture()
            .map_err(|e| e.to_string())
    }
}
