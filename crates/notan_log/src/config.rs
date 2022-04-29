use notan_app::{AppBuilder, Backend, BuildConfig};
use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use crate::console_error::console_error;

/// Configure the logs output
pub struct LogConfig {
    level: log::LevelFilter,
    levels_for: HashMap<String, log::LevelFilter>,
    colored: bool,
    date_format: String,
    use_utc: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        let level = if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Warn
        };

        let levels_for = if cfg!(target_arch = "wasm32") {
            HashMap::new()
        } else {
            vec![(String::from("winit"), log::LevelFilter::Warn)]
                .into_iter()
                .collect()
        };

        Self {
            level,
            levels_for,
            colored: cfg!(debug_assertions),
            date_format: String::from("%Y-%m-%d %H:%M:%S"),
            use_utc: false,
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

    pub fn debug() -> Self {
        Self::new(log::LevelFilter::Debug)
    }

    pub fn info() -> Self {
        Self::new(log::LevelFilter::Info)
    }

    pub fn warn() -> Self {
        Self::new(log::LevelFilter::Warn)
    }

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

    /// Set the date format (Defaults to "%Y-%m-%d %H:%M:%S")
    pub fn date_format(mut self, format: &str) -> Self {
        self.date_format = format.to_string();
        self
    }

    /// Set the date to UTC instead of Local
    pub fn use_utc(mut self) -> Self {
        self.use_utc = true;
        self
    }
}

impl<S, B> BuildConfig<S, B> for LogConfig
where
    B: Backend,
{
    fn apply(self, builder: AppBuilder<S, B>) -> AppBuilder<S, B> {
        let Self {
            level,
            levels_for,
            colored,
            date_format,
            use_utc,
        } = self;

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
                let date = if use_utc {
                    chrono::Utc::now().format(&date_format)
                } else {
                    chrono::Local::now().format(&date_format)
                };

                out.finish(format_args!(
                    "{date} [{target}] {level}: {message}",
                    date = date,
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
                let date = if use_utc {
                    chrono::Utc::now().format(&date_format)
                } else {
                    chrono::Local::now().format(&date_format)
                };

                out.finish(format_args!(
                    "{date} [{target}] {level}: {message}",
                    date = date,
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
    console_error(&format!("Error initializing logs: {}", e));
}

#[cfg(not(target_arch = "wasm32"))]
fn print_apply_error(e: &str) {
    println!("Error initializing logs: {}", e);
}
