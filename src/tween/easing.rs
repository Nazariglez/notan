use std::fmt::{Display, Formatter};

pub type EaseFn = fn(f32) -> f32;

#[inline]
pub fn interpolate(from: f32, to: f32, total_time: f32, elapsed_time: f32, easing: Easing) -> f32 {
    interpolate_with(from, to, total_time, elapsed_time, easing.into())
}

pub fn interpolate_with(
    from: f32,
    to: f32,
    total_time: f32,
    elapsed_time: f32,
    easing: EaseFn,
) -> f32 {
    from + ((to - from) * easing(elapsed_time / total_time))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Easing {
    Linear,
    InQuad,
    OutQuad,
    InOutQuad,
    InCubic,
    OutCubic,
    InOutCubic,
    InQuart,
    OutQuart,
    InOutQuart,
    InQuint,
    OutQuint,
    InOutQuint,
    InSine,
    OutSine,
    InOutSine,
    InExpo,
    OutExpo,
    InOutExpo,
    InCirc,
    OutCirc,
    InOutCirc,
    InElastic,
    OutElastic,
    InOutElastic,
    InBack,
    OutBack,
    InOutBack,
    InBounce,
    OutBounce,
    InOutBounce,
    Custom(EaseFn),
}

impl Into<EaseFn> for Easing {
    fn into(self) -> fn(f32) -> f32 {
        use Easing::*;
        match self {
            Custom(ease) => ease,
            Linear => linear,
            InQuad => in_quad,
            OutQuad => out_quad,
            InOutQuad => in_out_quad,
            InCubic => in_cubic,
            OutCubic => out_cubic,
            InOutCubic => in_out_cubic,
            InQuart => in_quart,
            OutQuart => out_quart,
            InOutQuart => in_out_quart,
            InQuint => in_quint,
            OutQuint => out_quint,
            InOutQuint => in_out_quint,
            InSine => in_sine,
            OutSine => out_sine,
            InOutSine => in_out_sine,
            InExpo => in_expo,
            OutExpo => out_expo,
            InOutExpo => in_out_expo,
            InCirc => in_circ,
            OutCirc => out_circ,
            InOutCirc => in_out_circ,
            InElastic => in_elastic,
            OutElastic => out_elastic,
            InOutElastic => in_out_elastic,
            InBack => in_back,
            OutBack => out_back,
            InOutBack => in_out_back,
            InBounce => in_bounce,
            OutBounce => out_bounce,
            InOutBounce => in_out_bounce,
        }
    }
}

impl Display for Easing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn linear(t: f32) -> f32 {
    t
}

pub fn in_quad(t: f32) -> f32 {
    t * t
}

pub fn out_quad(t: f32) -> f32 {
    t * (2.0 - t)
}

pub fn in_out_quad(t: f32) -> f32 {
    let mut t = t * 2.0;
    if t < 1.0 {
        return 0.5 * t * t;
    }

    t -= 1.0;

    -0.5 * (t * (t - 2.0) - 1.0)
}

pub fn in_cubic(t: f32) -> f32 {
    t * t * t
}

pub fn out_cubic(t: f32) -> f32 {
    in_cubic(t - 1.0) + 1.0
}

pub fn in_out_cubic(t: f32) -> f32 {
    let mut t = t * 2.0;
    if t < 1.0 {
        return 0.5 * t * t * t;
    }

    t -= 2.0;
    0.5 * (t * t * t + 2.0)
}

pub fn in_quart(t: f32) -> f32 {
    t * t * t * t
}

pub fn out_quart(t: f32) -> f32 {
    let t = t - 1.0;
    1.0 - (t * t * t * t)
}

pub fn in_out_quart(t: f32) -> f32 {
    let mut t = t * 2.0;
    if t < 1.0 {
        return 0.5 * t * t * t * t;
    }
    t -= 2.0;
    -0.5 * (t * t * t * t - 2.0)
}

pub fn in_quint(t: f32) -> f32 {
    t * t * t * t * t
}

pub fn out_quint(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t * t * t + 1.0
}

pub fn in_out_quint(t: f32) -> f32 {
    let mut t = t * 2.0;
    if t < 1.0 {
        return 0.5 * t * t * t * t * t;
    }

    t -= 2.0;
    0.5 * (t * t * t * t * t + 2.0)
}

pub fn in_sine(t: f32) -> f32 {
    1.0 - (t * std::f32::consts::PI / 2.0).cos()
}

pub fn out_sine(t: f32) -> f32 {
    (t * std::f32::consts::PI / 2.0).sin()
}

pub fn in_out_sine(t: f32) -> f32 {
    0.5 * (1.0 - (std::f32::consts::PI * t).cos())
}

pub fn in_expo(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else {
        (1024.0f32).powf(t - 1.0)
    }
}

pub fn out_expo(t: f32) -> f32 {
    if t == 1.0 {
        1.0
    } else {
        1.0 - (2.0f32).powf(-10.0 * t)
    }
}

pub fn in_out_expo(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }

    if t == 1.0 {
        return 1.0;
    }

    let t = t * 2.0;
    if t < 1.0 {
        return 0.5 * (1024f32).powf(t - 1.0);
    }

    0.5 * (-(2.0f32).powf(-10.0 * (t - 1.0)) + 2.0)
}

pub fn in_circ(t: f32) -> f32 {
    1.0 - (1.0 - t * t).sqrt()
}

pub fn out_circ(t: f32) -> f32 {
    let t = t - 1.0;
    (1.0 - (t * t)).sqrt()
}

pub fn in_out_circ(t: f32) -> f32 {
    let t = t * 2.0;
    if t < 1.0 {
        return -0.5 * ((1.0 - t * t).sqrt() - 1.0);
    }
    0.5 * ((1.0 - (t - 2.0) * (t - 2.0)).sqrt() + 1.0)
}

pub fn in_elastic(t: f32) -> f32 {
    if t == 0.0 || t == 1.0 {
        return t;
    }

    let a = 1.0;
    let p = 0.4;
    let s = p / 4.0;

    -(a * (2.0f32).powf(10.0 * (t - 1.0))
        * (((t - 1.0) - s) * (2.0 * std::f32::consts::PI) / p).sin())
}

pub fn out_elastic(t: f32) -> f32 {
    if t == 0.0 || t == 1.0 {
        return t;
    }

    let a = 1.0;
    let p = 0.4;
    let s = p / 4.0;

    (a * (2.0f32).powf(-10.0 * t) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin() + 1.0)
}

pub fn in_out_elastic(t: f32) -> f32 {
    if t == 0.0 || t == 1.0 {
        return t;
    }

    let a = 1.0;
    let p = 0.4;
    let s = p * (1.0f32 / a).asin() / (2.0 * std::f32::consts::PI);

    let t = t * 2.0;
    if t < 1.0 {
        -0.5 * (a
            * (2.0f32).powf(10.0 * (t - 1.0))
            * (((t - 1.0) - s) * (2.0 * std::f32::consts::PI) / p).sin())
    } else {
        a * (2.0f32).powf(-10.0 * (t - 1.0))
            * (((t - 1.0) - s) * (2.0 * std::f32::consts::PI) / p).sin()
            * 0.5
            + 1.0
    }
}

pub fn in_back(t: f32) -> f32 {
    let m = 1.70158;
    t * t * ((m + 1.0) * t - m)
}

pub fn out_back(t: f32) -> f32 {
    let t = t - 1.0;
    let m = 1.70158;
    t * t * ((m + 1.0) * t + m) + 1.0
}

pub fn in_out_back(t: f32) -> f32 {
    let m = 1.70158;
    let s = m * 1.525;
    let t = t * 2.0;
    if t < 1.0 {
        0.5 * (t * t * ((s + 1.0) * t - s))
    } else {
        0.5 * ((t - 2.0) * (t - 2.0) * ((s + 1.0) * (t - 2.0) + s) + 2.0)
    }
}

pub fn in_bounce(t: f32) -> f32 {
    1.0 - out_bounce(1.0 - t)
}

pub fn out_bounce(t: f32) -> f32 {
    let m = 2.75;
    let m1 = 7.5625;
    if t < (1.0 / m) {
        m1 * t * t
    } else if t < (2.0 / m) {
        let t = (t - (1.5 / m));
        m1 * t * t + 0.75
    } else if t < (2.5 / m) {
        let t = (t - (2.25 / m));
        m1 * t * t + 0.9375
    } else {
        let t = t - (2.625 / m);
        m1 * t * t + 0.984375
    }
}

pub fn in_out_bounce(t: f32) -> f32 {
    if t < 0.5 {
        in_bounce(t * 2.0) * 0.5
    } else {
        out_bounce(t * 2.0 - 1.0) * 0.5 + 0.5
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linear_interpolate_0() {
        let from = 0.0;
        let to = 100.0;
        let total_time = 10.0;
        let elapsed_time = 0.0;
        let value = interpolate(from, to, total_time, elapsed_time, Easing::Linear);
        assert_eq!(value, 0.0)
    }

    #[test]
    fn test_linear_interpolate_05() {
        let from = 0.0;
        let to = 100.0;
        let total_time = 10.0;
        let elapsed_time = 5.0;
        let value = interpolate(from, to, total_time, elapsed_time, Easing::Linear);
        assert_eq!(value, 50.0)
    }

    #[test]
    fn test_linear_interpolate_1() {
        let from = 0.0;
        let to = 100.0;
        let total_time = 10.0;
        let elapsed_time = 10.0;
        let value = interpolate(from, to, total_time, elapsed_time, Easing::Linear);
        assert_eq!(value, 100.0)
    }
}
