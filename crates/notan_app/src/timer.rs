use notan_utils::{Duration, Instant};
use std::collections::VecDeque;

/// Helper to measure and expose application's time events
#[derive(Debug, Clone)]
pub struct AppTimer {
    init_time: Instant,
    last_time: Option<Instant>,
    delta: Duration,
    delta_seconds: f32,
    elapsed: Duration,
    elapsed_time: f32,
    fps_cache: VecDeque<f32>,
    fps: f32,
}

impl Default for AppTimer {
    fn default() -> AppTimer {
        let fps = 60.0;

        // calculate the average fps for the last 60 frames
        let max_frames = 60;
        let mut fps_cache = VecDeque::with_capacity(max_frames);
        fps_cache.resize(max_frames, 1.0 / fps);

        AppTimer {
            init_time: Instant::now(),
            last_time: None,
            delta: Duration::from_secs(0),
            delta_seconds: 0.0,
            elapsed: Duration::from_secs(0),
            elapsed_time: 0.0,
            fps_cache,
            fps,
        }
    }
}

impl AppTimer {
    #[inline]
    pub(crate) fn update(&mut self) {
        let now = Instant::now();

        if let Some(last_time) = self.last_time {
            self.delta = now - last_time;
            self.delta_seconds = self.delta.as_secs_f32();
        }

        self.last_time = Some(now);

        self.elapsed = now - self.init_time;
        self.elapsed_time = self.elapsed.as_secs_f32();

        self.fps_cache.pop_front();
        self.fps_cache.push_back(self.delta_seconds);
        self.fps = 1.0 / (self.fps_cache.iter().sum::<f32>() / self.fps_cache.len() as f32);
    }

    /// Average frames per second (calculated using the last 60 frames)
    #[inline]
    pub fn fps(&self) -> f32 {
        self.fps
    }

    /// Delta time between frames
    #[inline]
    pub fn delta(&self) -> Duration {
        self.delta
    }

    /// Delta time between frames in seconds
    #[inline]
    pub fn delta_f32(&self) -> f32 {
        self.delta_seconds
    }

    /// Elapsed time since application's init
    #[inline]
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Elapsed time since application's init in seconds
    #[inline]
    pub fn elapsed_f32(&self) -> f32 {
        self.elapsed_time
    }

    /// Application's init time
    #[inline]
    pub fn init_time(&self) -> Instant {
        self.init_time
    }

    /// Last frame time
    #[inline]
    pub fn last_time(&self) -> Option<Instant> {
        self.last_time
    }
}
