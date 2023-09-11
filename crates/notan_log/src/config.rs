use notan_app::{AppBuilder, Backend, BuildConfig};
use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use crate::console_error::console_error;

/// Configure the logs output
/// Logs will show a timestamp using the UTC time with format `[year]-[month]-[day] [hour]:[minutes]:[seconds]`
#[derive(Clone)]
pub struct LogConfig {
    level: log::LevelFilter,
    levels_for: HashMap<String, log::LevelFilter>,
    colored: bool,
    verbose: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        let level = if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Warn
        };

        Self {
            level,
            levels_for: Default::default(),
            colored: cfg!(debug_assertions),
            verbose: false,
        }
    }
}

impl LogConfig {
    /// Creates a new configuration using the given level filter
    pub fn new(level: log::LevelFilter) -> Self {
        Self {
            level,
            ..Default::default()
        }
    }

    /// Configure logs to use trace level filter
    pub fn trace() -> Self {
        Self::new(log::LevelFilter::Trace)
    }

    /// Configure logs to use debug level filter
    pub fn debug() -> Self {
        Self::new(log::LevelFilter::Debug)
    }

    /// Configure logs to use info level filter
    pub fn info() -> Self {
        Self::new(log::LevelFilter::Info)
    }

    /// Configure logs to use warn level filter
    pub fn warn() -> Self {
        Self::new(log::LevelFilter::Warn)
    }

    /// Configure logs to use error level filter
    pub fn error() -> Self {
        Self::new(log::LevelFilter::Error)
    }

    /// Changes the level filter
    pub fn level(mut self, level: log::LevelFilter) -> Self {
        self.level = level;
        self
    }

    /// Change the filter level for dependencies
    pub fn level_for(mut self, id: &str, level: log::LevelFilter) -> Self {
        self.levels_for.insert(id.to_string(), level);
        self
    }

    /// Enable colored text (Defaults to true on debug mode)
    pub fn use_colors(mut self, value: bool) -> Self {
        self.colored = value;
        self
    }

    /// Log everything including dependencies
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}

impl<S, B> BuildConfig<S, B> for LogConfig
where
    B: Backend,
{
    fn apply(&self, builder: AppBuilder<S, B>) -> AppBuilder<S, B> {
        let Self {
            level,
            mut levels_for,
            colored,
            verbose,
        } = self.clone();

        if !verbose {
            let mut disabled = vec![
                "symphonia_core",
                "symphonia_codec_vorbis",
                "symphonia_format_ogg",
            ];

            if !cfg!(target_arch = "wasm32") {
                disabled.push("winit");
            }

            disabled.iter().for_each(|id| {
                levels_for.insert(id.to_string(), log::LevelFilter::Warn);
            });
        }

        let mut dispatch = fern::Dispatch::new().level(level);

        for (id, lvl) in levels_for.iter() {
            dispatch = dispatch.level_for(id.clone(), *lvl);
        }

        dispatch = chain_output(dispatch);

        if colored {
            use fern::colors::{Color, ColoredLevelConfig};

            let color_level = ColoredLevelConfig::new()
                .error(Color::BrightRed)
                .warn(Color::BrightYellow)
                .info(Color::BrightGreen)
                .debug(Color::BrightCyan)
                .trace(Color::BrightBlack);

            dispatch = dispatch.format(move |out, message, record| {
                out.finish(format_args!(
                    "{date} [{target}] {level}: {message}",
                    date = get_time(),
                    target = record.target(),
                    level = format_args!(
                        "{}\x1b[{}m",
                        color_level.color(record.level()),
                        Color::White.to_fg_str()
                    ),
                    message = message,
                ))
            });
        } else {
            dispatch = dispatch.format(move |out, message, record| {
                out.finish(format_args!(
                    "{date} [{target}] {level}: {message}",
                    date = get_time(),
                    target = record.target(),
                    level = record.level(),
                    message = message,
                ))
            });
        }

        if let Err(e) = dispatch.apply() {
            print_apply_error(&e.to_string());
        }

        builder
    }

    fn late_evaluation(&self) -> bool {
        true
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_time() -> String {
    let format =
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    time::OffsetDateTime::now_utc().format(&format).unwrap()
}

#[cfg(target_arch = "wasm32")]
fn get_time() -> String {
    let now = js_sys::Date::new_0();
    format!(
        "{}-{}-{} {}:{}:{}",
        now.get_utc_full_year(),
        now.get_utc_month(),
        now.get_utc_day(),
        now.get_utc_hours(),
        now.get_utc_minutes(),
        now.get_utc_seconds()
    )
}

#[cfg(target_arch = "wasm32")]
fn chain_output(dispatch: fern::Dispatch) -> fern::Dispatch {
    dispatch.chain(fern::Output::call(console_log::log))
}

#[cfg(not(target_arch = "wasm32"))]
fn chain_output(dispatch: fern::Dispatch) -> fern::Dispatch {
    dispatch.chain(std::io::stdout())
}

#[cfg(target_arch = "wasm32")]
fn print_apply_error(e: &str) {
    console_error(&format!("Error initializing logs: {e}"));
}

#[cfg(not(target_arch = "wasm32"))]
fn print_apply_error(e: &str) {
    println!("Error initializing logs: {e}");
}
