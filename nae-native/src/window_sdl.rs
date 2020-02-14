use crate::System;
use nae_core::window::BaseWindow;
use nae_core::{BaseApp, BuilderOpts};
use sdl2::video::{FullscreenType, Window as SdlWindow};
use sdl2::{Sdl, VideoSubsystem};

pub struct Window {
    pub(crate) sdl: sdl2::Sdl,
    pub(crate) video: VideoSubsystem,
    pub(crate) win: SdlWindow,
}

impl Window {
    pub(crate) fn new(opts: &BuilderOpts) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let mut win_builder = video.window(&opts.title, opts.width as _, opts.height as _);

        win_builder.opengl();

        if opts.resizable {
            win_builder.resizable();
        }

        if opts.maximized {
            win_builder.maximized();
        }

        if opts.fullscreen {
            win_builder.fullscreen();
        }

        // TODO add all funcionality like min_size or max_size

        let mut win = win_builder.build().map_err(|e| e.to_string())?;

        if let Some((width, height)) = opts.min_size {
            win.set_minimum_size(width as _, height as _);
        }

        if let Some((width, height)) = opts.max_size {
            win.set_maximum_size(width as _, height as _);
        }

        dbg!(video.display_dpi(0));

        Ok(Self { sdl, video, win })
    }

    pub(crate) fn set_fullscreen(&mut self, full: bool) {
        let state = if full {
            FullscreenType::True
        } else {
            FullscreenType::Off
        };
        self.win.set_fullscreen(state);
    }
}

impl BaseWindow for Window {
    fn width(&self) -> i32 {
        let (width, _) = self.win.size();
        width as _
    }

    fn height(&self) -> i32 {
        let (_, height) = self.win.size();
        height as _
    }

    fn fullscreen(&self) -> bool {
        match self.win.fullscreen_state() {
            FullscreenType::True => true,
            _ => false,
        }
    }

    fn title(&self) -> &str {
        self.win.title()
    }

    fn dpi(&self) -> f32 {
        //TODO set the real value
        1.0
    }
}

pub fn run<A, S, F, D>(mut app: A, mut state: S, mut update: F, mut draw: D) -> Result<(), String>
where
    A: BaseApp<System = System> + 'static,
    S: 'static,
    F: FnMut(&mut A, &mut S) + 'static,
    D: FnMut(&mut A, &mut S) + 'static,
{
    use sdl2::event::Event as SdlEvent;
    let mut event_pump = app
        .system()
        .window
        .sdl
        .event_pump()
        .map_err(|e| e.to_string())?;

    let mut running = true;
    while running {
        for evt in event_pump.poll_iter() {
            println!("{:?}", evt);
            match evt {
                SdlEvent::Quit { .. } => running = false,
                _ => {}
            }
        }
    }
    Ok(())
}
