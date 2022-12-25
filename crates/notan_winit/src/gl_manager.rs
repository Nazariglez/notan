use glutin::config::{Config, ConfigTemplateBuilder, GlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, GlProfile, NotCurrentGlContextSurfaceAccessor,
    PossiblyCurrentContext, Version,
};
use glutin::display::{Display, GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, Surface, SurfaceAttributesBuilder, SwapInterval, WindowSurface};
use glutin_winit::DisplayBuilder;
use notan_app::WindowConfig;
use raw_window_handle::HasRawWindowHandle;
use std::num::NonZeroU32;
use winit::event_loop::EventLoop;
use winit::window::Fullscreen::Borderless;
use winit::window::{Window, WindowBuilder};

pub(crate) struct GlManager {
    pub surface: Surface<WindowSurface>,
    pub window: Window,
    pub display: Display,
    pub context: PossiblyCurrentContext,
}

impl GlManager {
    pub fn new(
        builder: WindowBuilder,
        event_loop: &EventLoop<()>,
        config: &WindowConfig,
    ) -> Result<Self, String> {
        let mut template = ConfigTemplateBuilder::new().with_transparency(config.transparent);

        if config.multisampling > 0 {
            if !config.multisampling.is_power_of_two() {
                return Err("Multisampling must be a power of two.".to_string());
            }

            template = template.with_multisampling(config.multisampling);
        }

        let (window, gl_config) = DisplayBuilder::new()
            .with_window_builder(Some(builder))
            .build(event_loop, template, get_config(config))
            .map_err(|e| {
                let mut err = String::from("Cannot select a valid OpenGL configuration");
                if config.multisampling != 0 {
                    err.push_str(", try to reduce the number of samples");
                }
                format!("{}: {}", err, e)
            })?;

        let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());
        let display = gl_config.display();

        let context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(raw_window_handle);

        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(Some(Version::new(3, 0))))
            .build(raw_window_handle);

        let mut not_current_gl_context = Some(unsafe {
            display
                .create_context(&gl_config, &context_attributes)
                .or_else(|_| display.create_context(&gl_config, &fallback_context_attributes))
                .map_err(|e| e.to_string())?
        });

        let window =
            window.ok_or_else(|| "Cannot create a Window for the GL Context.".to_string())?;
        let (width, height): (u32, u32) = window.inner_size().into();
        let raw_window_handle = window.raw_window_handle();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new()
            .with_srgb(Some(true))
            .build(
                raw_window_handle,
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            );

        let surface = unsafe {
            display
                .create_window_surface(&gl_config, &attrs)
                .map_err(|e| e.to_string())?
        };

        // Make it current.
        let context = not_current_gl_context
            .take()
            .ok_or_else(|| "Cannot create GL Context".to_string())?
            .make_current(&surface)
            .map_err(|e| e.to_string())?;

        Ok(Self {
            surface,
            window,
            display,
            context,
        })
    }

    pub fn enable_vsync(&self, enable: bool) -> Result<(), String> {
        let interval = if enable {
            SwapInterval::Wait(NonZeroU32::new(1).unwrap())
        } else {
            SwapInterval::DontWait
        };

        self.surface
            .set_swap_interval(&self.context, interval)
            .map_err(|e| e.to_string())
    }

    pub fn set_cursor_hittest(&self, enable: bool) -> Result<(), String> {
        self.window
            .set_cursor_hittest(enable)
            .map_err(|e| e.to_string())
    }

    pub fn scale_factor(&self) -> f64 {
        self.window
            .current_monitor()
            .as_ref()
            .map_or(1.0, |m| m.scale_factor())
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        if fullscreen {
            let monitor = self.window.current_monitor();
            self.window.set_fullscreen(Some(Borderless(monitor)));
        } else {
            self.window.set_fullscreen(None);
        }
    }

    pub fn swap_buffers(&self) -> Result<(), String> {
        self.surface
            .swap_buffers(&self.context)
            .map_err(|e| e.to_string())
    }

    pub fn resize(&self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.surface.resize(
            &self.context,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );
    }
}

#[cfg(not(feature = "nvidia-wayland"))]
fn get_config(
    config: &WindowConfig,
) -> impl FnOnce(Box<dyn Iterator<Item = Config> + '_>) -> Config + '_ {
    |configs| {
        configs
            .reduce(|accum, conf| {
                let next_srgb = conf.srgb_capable();
                let next_transparency = conf.supports_transparency().unwrap_or(false);
                let more_samples = conf.num_samples() > accum.num_samples();

                // value of transparency for the priority check
                let transparency_check = if config.transparent {
                    next_transparency
                } else {
                    true
                };

                // priority 1: supports srgba, transparency and has more samples than current one
                let full_support = next_srgb && transparency_check && more_samples;

                // priority 2: we don't care about transparency if it's not supported by next config
                let srgba_plus_samples = next_srgb && more_samples;

                // priority 3: if it supports srgba is enough
                let only_srgba = next_srgb;

                // select the config in order of priority
                let select_config = full_support || srgba_plus_samples || only_srgba;

                if select_config {
                    conf
                } else {
                    accum
                }
            })
            .unwrap()
    }
}
#[cfg(feature = "nvidia-wayland")]
fn get_config(
    _config: &WindowConfig,
) -> impl FnOnce(Box<dyn Iterator<Item = Config> + '_>) -> Config + '_ {
    |configs| {
        configs
            .reduce(|accum, conf| {
                let transparency_check = conf.supports_transparency().unwrap_or(false)
                    & !accum.supports_transparency().unwrap_or(false);

                if transparency_check || conf.num_samples() > accum.num_samples() {
                    conf
                } else {
                    accum
                }
            })
            .unwrap()
    }
}
