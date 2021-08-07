use crate::window::WinitWindowBackend;
use glutin::event_loop::ControlFlow;
use notan_app::buffer::VertexAttr;
use notan_app::commands::Commands;
use notan_app::config::WindowConfig;
use notan_app::graphics::pipeline::PipelineOptions;
use notan_app::{
    App, Backend, BackendSystem, DeviceBackend, EventIterator, InitializeFn, ResourceId,
    TextureInfo, TextureUpdate, WindowBackend,
};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;

pub struct WinitBackend {
    window: Option<WinitWindowBackend>,
    exit_requested: bool,
}

impl WinitBackend {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            window: None,
            exit_requested: false,
        })
    }
}

impl Backend for WinitBackend {
    fn window(&mut self) -> &mut dyn WindowBackend {
        self.window.as_mut().unwrap()
    }

    fn events_iter(&mut self) -> EventIterator {
        Default::default()
    }

    fn exit(&mut self) {
        self.exit_requested = true;
    }
}

impl BackendSystem for WinitBackend {
    fn initialize<S, R>(&mut self, window: WindowConfig) -> Result<Box<InitializeFn<S, R>>, String>
    where
        S: 'static,
        R: FnMut(&mut App, &mut S) -> Result<(), String> + 'static,
    {
        let event_loop = EventLoop::new();
        self.window = Some(WinitWindowBackend::new(window, &event_loop)?);

        Ok(Box::new(move |mut app: App, mut state: S, mut cb: R| {
            event_loop.run(move |event, _win_target, control_flow| {
                match event {
                    Event::WindowEvent { ref event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            app.exit();
                        }
                        _ => {}
                    },
                    Event::MainEventsCleared => {
                        backend(&mut app)
                            .window
                            .as_mut()
                            .unwrap()
                            .window
                            .request_redraw();
                    }
                    Event::RedrawRequested(_) => {
                        cb(&mut app, &mut state);
                    }
                    _ => {}
                }

                let exit_requested = backend(&mut app).exit_requested;
                if exit_requested {
                    *control_flow = ControlFlow::Exit;
                }
            });
        }))
    }

    fn get_graphics_backend(&self) -> Box<DeviceBackend> {
        Box::new(WinitDeviceBackend::default())
    }
}

#[derive(Default)]
struct WinitDeviceBackend {
    id_count: i32,
}

impl DeviceBackend for WinitDeviceBackend {
    fn api_name(&self) -> &str {
        "opengl"
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

    fn create_uniform_buffer(&mut self, _slot: u32, name: &str) -> Result<i32, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_render_texture(
        &mut self,
        _texture_id: i32,
        _info: &TextureInfo,
    ) -> Result<i32, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_texture(&mut self, info: &TextureInfo) -> Result<i32, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn render(&mut self, commands: &[Commands], _target: Option<i32>) {}

    fn clean(&mut self, to_clean: &[ResourceId]) {}

    fn set_size(&mut self, width: i32, height: i32) {}

    fn update_texture(&mut self, texture: i32, opts: &TextureUpdate) -> Result<(), String> {
        Ok(())
    }
}

fn backend(app: &mut App) -> &mut WinitBackend {
    app.backend.downcast_mut::<WinitBackend>().unwrap()
}
