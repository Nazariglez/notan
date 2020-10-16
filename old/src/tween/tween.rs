use super::easing::*;

#[derive(Debug)]
pub struct Tween {
    pub from: f32,
    pub to: f32,
    pub time_range: f32,
    pub easing: Easing,
    pub delay: f32,
    pub repeat_forever: bool,
    pub repeat_times: u32,
    pub use_yoyo: bool,

    pub total_time: f32,
    pub value: f32,

    is_active: bool,
    did_finish: bool,

    elapsed_delay: f32,
    already_repeated: u32,
    yoyo_back: bool,
    elapsed_time: f32,
}

impl Tween {
    pub fn new(from: f32, to: f32, time_range: f32) -> Self {
        let easing = Easing::Linear;
        let value = from;
        let delay = 0.0;
        let repeat_forever = false;
        let yoyo = false;
        let repeat_times = 0;

        Self {
            from,
            to,
            time_range,
            easing,
            value,
            delay,
            repeat_forever,
            repeat_times,
            use_yoyo: yoyo,

            elapsed_time: 0.0,
            elapsed_delay: 0.0,
            total_time: 0.0,
            already_repeated: 0,
            is_active: false,
            did_finish: false,
            yoyo_back: false,
        }
    }

    pub fn did_finish(&self) -> bool {
        self.did_finish
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn already_repeated(&self) -> u32 {
        self.already_repeated
    }

    pub fn reset(&mut self) -> &mut Self {
        self.elapsed_time = 0.0;
        self.elapsed_delay = 0.0;
        self.total_time = 0.0;
        self.already_repeated = 0;
        self.did_finish = false;
        self.yoyo_back = false;
        self
    }

    pub fn tick(&mut self, delta: f32) {
        if !can_update(self) {
            return;
        }

        if self.delay > self.elapsed_delay {
            self.elapsed_delay += delta;
            return;
        }

        let time = if self.use_yoyo {
            self.time_range * 0.5
        } else {
            self.time_range
        };

        if time > self.elapsed_time {
            let t = self.elapsed_time + delta;
            let ended = t >= time;

            self.elapsed_time = if ended { time } else { t };

            self.value = interpolate(
                self.from,
                self.to,
                time,
                self.elapsed_time,
                self.easing.into(),
            );

            self.total_time = if self.yoyo_back {
                time + self.elapsed_time
            } else {
                self.elapsed_time
            };

            if ended {
                if self.use_yoyo && !self.yoyo_back {
                    self.yoyo_back = true;
                    std::mem::swap(&mut self.from, &mut self.to);
                    self.elapsed_time = 0.0;
                    return;
                }

                if self.repeat_forever || self.repeat_times > self.already_repeated {
                    self.already_repeated += 1;
                    self.elapsed_time = 0.0;

                    if self.use_yoyo && self.yoyo_back {
                        std::mem::swap(&mut self.from, &mut self.to);
                        self.yoyo_back = false;
                    }
                    return;
                }

                self.did_finish = true;
                self.is_active = false;
            }
        }
    }

    pub fn start(&mut self) -> &mut Self {
        self.is_active = true;
        self
    }

    pub fn stop(&mut self) -> &mut Self {
        self.is_active = false;
        self
    }
}

#[inline]
fn can_update(tween: &Tween) -> bool {
    tween.time_range > 0.0 && tween.is_active
}
