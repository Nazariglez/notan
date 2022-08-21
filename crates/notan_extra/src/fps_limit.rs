use notan_app::{assets::Assets, App, AppFlow, Graphics, Plugin};

/// Limit the App frame rate to a maximum
pub struct FpsLimit {
    target: u8,
    seconds: f64,
    elapsed: f64,
}

impl FpsLimit {
    pub fn new(target: u8) -> Self {
        let fps = target as f64;
        let seconds = 1.0 / fps;
        Self {
            target,
            seconds,
            elapsed: 0.0,
        }
    }

    pub fn target(&self) -> u8 {
        self.target
    }
}

impl Plugin for FpsLimit {
    // Wasm will run as fast as it can because the browser
    // will send requestAnimationFrame all the time
    // what we do to limit the framerate is just skipping frames
    #[cfg(target_arch = "wasm32")]
    fn pre_frame(
        &mut self,
        app: &mut App,
        _assets: &mut Assets,
        _gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        self.elapsed += app.system_timer.delta().as_secs_f64();
        if self.elapsed >= self.seconds {
            self.elapsed = 0.0;
            Ok(AppFlow::Next)
        } else {
            Ok(AppFlow::SkipFrame)
        }
    }

    // On native platforms like desktop we can sleep the thread
    // reducing the use of the cpu.
    // We need spin_sleep because thread::sleep is not really accurate
    // and spin_sleep help us to be accurate mixing sleep + spin
    #[cfg(not(target_arch = "wasm32"))]
    fn pre_frame(
        &mut self,
        app: &mut App,
        _assets: &mut Assets,
        _gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        self.elapsed += app.system_timer.delta().as_secs_f64();
        if self.elapsed < self.seconds {
            let wait = self.seconds - self.elapsed;
            spin_sleep::sleep(std::time::Duration::from_secs_f64(wait));
        }

        self.elapsed = 0.0;
        return Ok(AppFlow::Next);
    }
}
