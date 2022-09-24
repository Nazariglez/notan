use notan_app::{assets::Assets, App, AppFlow, Graphics, Plugin};

const IS_WASM: bool = cfg!(target_arch = "wasm32");

/// Limit the App frame rate to a maximum
pub struct FpsLimit {
    limit: u8,
    seconds: f64,
    elapsed: f64,
    sleep: bool,
}

impl FpsLimit {
    pub fn new(limit: u8) -> Self {
        let fps = limit as f64;
        let seconds = 1.0 / fps;
        Self {
            limit,
            seconds,
            elapsed: 0.0,
            sleep: true,
        }
    }

    /// Returns the fps limit
    pub fn limit(&self) -> u8 {
        self.limit
    }

    /// Set if the thread should wait sleeping or not (only native platforms)
    pub fn sleep(mut self, sleep: bool) -> Self {
        self.sleep = sleep;
        self
    }
}

impl Plugin for FpsLimit {
    // Wasm will run as fast as it can because the browser
    // will send requestAnimationFrame all the time
    // what we do to limit the framerate is just skipping frames
    fn pre_frame(
        &mut self,
        app: &mut App,
        _assets: &mut Assets,
        _gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        // If sleep is disabled then native platforms will use this too
        let can_skip_frame = IS_WASM || !self.sleep;
        if !can_skip_frame {
            return Ok(AppFlow::Next);
        }

        self.elapsed += app.system_timer.delta().as_secs_f64();
        if self.elapsed >= self.seconds {
            self.elapsed -= self.seconds;
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
    fn post_frame(
        &mut self,
        app: &mut App,
        _assets: &mut Assets,
        _gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        // if sleep is disabled the pre_frame method will manage
        // what frames are skipped
        if !self.sleep {
            return Ok(AppFlow::Next);
        }

        // if sleep is enabled put the thread to sleep when needed to achieve the limit
        self.elapsed += app.system_timer.delta().as_secs_f64();
        let wait = self.seconds - self.elapsed;
        if wait > 0.0 {
            spin_sleep::sleep(std::time::Duration::from_secs_f64(wait));
        }

        self.elapsed -= self.seconds;
        Ok(AppFlow::Next)
    }
}
