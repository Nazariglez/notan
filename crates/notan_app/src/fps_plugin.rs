use crate::assets::Assets;
use crate::{App, AppFlow, Graphics, Plugin};

/// Limit the App frame rate to a maximum
pub struct FpsPlugin {
    target: u8,
    seconds: f64,
    elapsed: f64,
}

impl FpsPlugin {
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

impl Plugin for FpsPlugin {
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
}
