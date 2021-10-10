use crate::{App, AppFlow, Plugin};

/// Limit the App frame rate to a maximum
pub struct FpsPlugin {
    fps: f64,
    seconds: f64,
    elapsed: f64,
}

impl FpsPlugin {
    pub fn new(target: u8) -> Self {
        let fps = target as f64;
        let seconds = 1.0 / fps;
        Self {
            fps,
            seconds,
            elapsed: 0.0,
        }
    }
}

impl Plugin for FpsPlugin {
    fn pre_frame(&mut self, app: &mut App) -> Result<AppFlow, String> {
        self.elapsed += app.system_timer.delta().as_secs_f64();
        if self.elapsed >= self.seconds {
            self.elapsed = 0.0;
            Ok(AppFlow::Next)
        } else {
            Ok(AppFlow::SkipFrame)
        }
    }
}
