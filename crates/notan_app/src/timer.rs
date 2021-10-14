use notan_utils::{Duration, Instant};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct AppTimer {
    init_time: Instant,
    last_time: Option<Instant>,
    delta: Duration,
    delta_seconds: f32,
    time_since_init: f32,
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
            time_since_init: 0.0,
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

        let time_since_init = now - self.init_time;
        self.time_since_init = time_since_init.as_secs_f32();

        self.fps_cache.pop_front();
        self.fps_cache.push_back(self.delta_seconds);
        self.fps = 1.0 / (self.fps_cache.iter().sum::<f32>() / self.fps_cache.len() as f32);
    }

    #[inline]
    pub fn fps(&self) -> f32 {
        self.fps
    }

    #[inline]
    pub fn delta(&self) -> Duration {
        self.delta
    }

    #[inline]
    pub fn delta_f32(&self) -> f32 {
        self.delta_seconds
    }

    #[inline]
    pub fn time_since_init(&self) -> f32 {
        self.time_since_init
    }

    #[inline]
    pub fn init_time(&self) -> Instant {
        self.init_time
    }

    #[inline]
    pub fn last_time(&self) -> Option<Instant> {
        self.last_time
    }
}
