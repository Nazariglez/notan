use crate::config::WindowConfig;
// use crate::graphics::{Device, DeviceBackend, RenderTexture};
use crate::{App, Backend, BackendSystem, EventIterator, FrameState, InitializeFn, WindowBackend};
use notan_graphics::prelude::*;

#[derive(Default)]
pub struct EmptyWindowBackend {
    size: (i32, i32),
    is_fullscreen: bool,
}

impl WindowBackend for EmptyWindowBackend {
    fn set_size(&mut self, width: i32, height: i32) {
        self.size = (width, height);
    }

    fn size(&self) -> (i32, i32) {
        self.size
    }

    fn set_fullscreen(&mut self, enabled: bool) {
        self.is_fullscreen = enabled;
    }

    fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }

    fn dpi(&self) -> f64 {
        1.0
    }
}

#[derive(Default)]
pub struct EmptyBackend {
    window: EmptyWindowBackend,
}

impl EmptyBackend {
    pub fn new() -> Result<Self, String> {
        Ok(Default::default())
    }
}

impl Backend for EmptyBackend {
    fn window(&mut self) -> &mut dyn WindowBackend {
        &mut self.window
    }

    fn events_iter(&mut self) -> EventIterator {
        Default::default()
    }

    fn exit(&mut self) {}

    fn system_timestamp(&self) -> u64 {
        0
    }
}

impl BackendSystem for EmptyBackend {
    fn initialize<S, R>(&mut self, _config: WindowConfig) -> Result<Box<InitializeFn<S, R>>, String>
    where
        S: 'static,
        R: FnMut(&mut App, &mut S) -> Result<FrameState, String> + 'static,
    {
        Ok(Box::new(|mut app: App, mut state: S, mut cb: R| {
            // This function should block with a loop or raf in the platform specific backends
            // while !app.closed {
            if let Err(e) = cb(&mut app, &mut state) {
                log::error!("{}", e);
            }
            // }
            Ok(())
        }))
    }

    fn get_graphics_backend(&self) -> Box<dyn DeviceBackend> {
        Box::new(EmptyDeviceBackend::default())
    }
}

#[derive(Default)]
struct EmptyDeviceBackend {
    id_count: u64,
}

impl DeviceBackend for EmptyDeviceBackend {
    fn api_name(&self) -> &str {
        ""
    }

    fn create_pipeline(
        &mut self,
        _vertex_source: &[u8],
        _fragment_source: &[u8],
        _vertex_attrs: &[VertexAttr],
        _options: PipelineOptions,
    ) -> Result<u64, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_vertex_buffer(
        &mut self,
        _attrs: &[VertexAttr],
        _step_mode: VertexStepMode,
    ) -> Result<u64, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_index_buffer(&mut self) -> Result<u64, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_uniform_buffer(&mut self, _slot: u32, _name: &str) -> Result<u64, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn render(&mut self, commands: &[Commands], _target: Option<u64>) {
        commands.iter().for_each(|cmd| log::info!("{:?}", cmd));
    }

    fn clean(&mut self, to_clean: &[ResourceId]) {
        log::info!("{:?}", to_clean);
    }

    fn set_size(&mut self, _width: i32, _height: i32) {}

    fn set_dpi(&mut self, _scale_factor: f64) {}

    fn create_texture(&mut self, _info: &TextureInfo) -> Result<u64, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_render_texture(
        &mut self,
        _texture_id: u64,
        _info: &TextureInfo,
    ) -> Result<u64, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn update_texture(&mut self, _texture: u64, _opts: &TextureUpdate) -> Result<(), String> {
        Ok(())
    }

    fn read_pixels(
        &mut self,
        _texture: u64,
        _bytes: &mut [u8],
        _opts: &TextureRead,
    ) -> Result<(), String> {
        Ok(())
    }
}
