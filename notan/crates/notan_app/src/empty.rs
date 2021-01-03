use crate::config::WindowConfig;
use crate::graphics::prelude::*;
use crate::graphics::{Graphics, GraphicsBackend, RenderTarget};
use crate::{App, Backend, BackendSystem, EventIterator, InitializeFn, WindowBackend};

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
}

#[derive(Default)]
pub struct EmptyBackend {
    exit_requested: bool,
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

    fn exit(&mut self) {
        self.exit_requested = true;
    }
}

impl BackendSystem for EmptyBackend {
    fn initialize<S, R>(&mut self, _config: WindowConfig) -> Result<Box<InitializeFn<S, R>>, String>
    where
        S: 'static,
        R: FnMut(&mut App, &mut S) -> Result<(), String> + 'static,
    {
        Ok(Box::new(|mut app: App, mut state: S, mut cb: R| {
            // This function should block with a loop or raf in the platform specific backends
            cb(&mut app, &mut state)
        }))
    }

    fn get_graphics_backend(&self) -> Box<GraphicsBackend> {
        Box::new(EmptyGraphicsBackend::default())
    }
}

#[derive(Default)]
struct EmptyGraphicsBackend {
    id_count: i32,
}

impl GraphicsBackend for EmptyGraphicsBackend {
    fn api_name(&self) -> &str {
        ""
    }

    fn create_pipeline(
        &mut self,
        vertex_source: &[u8],
        fragment_source: &[u8],
        vertex_attrs: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<i32, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_vertex_buffer(&mut self) -> Result<i32, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_index_buffer(&mut self) -> Result<i32, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_uniform_buffer(&mut self, _slot: u32) -> Result<i32, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_render_target(&mut self, _texture_id: i32) -> Result<i32, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_texture(&mut self, info: &TextureInfo) -> Result<i32, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn render(&mut self, commands: &[Commands], _target: Option<i32>) {
        commands
            .iter()
            .for_each(|cmd| notan_log::info!("{:?}", cmd));
    }

    fn clean(&mut self, to_clean: &[ResourceId]) {
        notan_log::info!("{:?}", to_clean);
    }

    fn set_size(&mut self, width: i32, height: i32) {}
}
