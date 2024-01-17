#[macro_export]
macro_rules! option_usize_env {
    ($s:expr, $d:expr) => {
        $crate::utils::parse_string_as_usize(option_env!($s), $d)
    };
}

pub const fn parse_string_as_usize(key: Option<&'static str>, default: usize) -> usize {
    match key {
        None => default, // Default value
        Some(num) => {
            // str.parse::<usize>() is not a const fn yet
            // this trick will do it for now:
            // https://www.reddit.com/r/rust/comments/10ol38k/comment/j6fbjwj/?utm_source=reddit&utm_medium=web2x&context=3
            let mut res: usize = 0;
            let mut bytes = num.as_bytes();
            while let [byte, rest @ ..] = bytes {
                bytes = rest;
                if let b'0'..=b'9' = byte {
                    res *= 10;
                    res += (*byte - b'0') as usize;
                } else {
                    // FIXME: log here that the value cannot be parsed?
                    return default;
                }
            }
            res
        }
    }
}
