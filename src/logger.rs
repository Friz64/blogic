use ansi_term::Color::*;
use chrono::{Datelike, Local, Timelike};
use log::{error, info, Level, LevelFilter, Log, Metadata, Record};
use std::{
    fmt::Display,
    fs::{self, File, OpenOptions},
    io::{ErrorKind, Write},
    process,
    string::ToString,
    sync::Mutex,
};

pub mod prelude {
    pub use super::UnwrapLog;
    pub use log::{error, info, warn};
}

pub trait UnwrapLog<T> {
    fn unwrap_log(self, message: impl Display) -> T;
}

impl<T, E> UnwrapLog<T> for Result<T, E>
where
    E: Display,
{
    fn unwrap_log(self, message: impl Display) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                error!("{}: {}", message, err);
                process::exit(1)
            }
        }
    }
}

impl<T> UnwrapLog<T> for Option<T> {
    fn unwrap_log(self, message: impl Display) -> T {
        match self {
            Some(val) => val,
            None => {
                error!("{}", message);
                process::exit(1)
            }
        }
    }
}

pub struct Logger {
    latest_log: Mutex<File>,
    archived_log: Mutex<File>,
    blacklist: Vec<String>,
    console_colored: bool,
}

impl Logger {
    pub fn init(console_colored: bool, blacklist: &[&str]) {
        if let Err(err) = fs::create_dir("logs") {
            match err.kind() {
                ErrorKind::AlreadyExists => (),
                _ => Err(err).expect("Failed to create logs folder"),
            }
        }

        let latest_log = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("latest_log.txt")
            .expect("Failed to create lastest log");

        let time_log_path = format!("./logs/{}-log.txt", format_yyyymmdd_hhmmss());
        let time_log = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&time_log_path)
            .expect("Failed to create archived log");

        let logger = Logger {
            latest_log: Mutex::new(latest_log),
            archived_log: Mutex::new(time_log),
            blacklist: blacklist.iter().map(ToString::to_string).collect(),
            console_colored,
        };

        if log::set_boxed_logger(Box::new(logger)).is_ok() {
            log::set_max_level(LevelFilter::Info)
        }

        info!("Logger initialized");
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        let module_path = record.module_path().unwrap_or_default();
        let blacklisted = self
            .blacklist
            .iter()
            .any(|entry| module_path.contains(entry));

        if self.enabled(record.metadata()) && !blacklisted {
            let time = format_hhmmssnnn();
            let uncolored_output = output(&time, record, false);

            let _ = writeln!(self.latest_log.lock().unwrap(), "{}", uncolored_output);
            let _ = writeln!(self.archived_log.lock().unwrap(), "{}", uncolored_output);

            let console_ouput = if self.console_colored {
                output(&time, record, true)
            } else {
                uncolored_output
            };

            println!("{}", console_ouput);
        }
    }

    fn flush(&self) {
        if let Ok(mut file) = self.latest_log.lock() {
            let _ = file.flush();
        }

        if let Ok(mut file) = self.archived_log.lock() {
            let _ = file.flush();
        }
    }
}

fn output(time: &str, record: &log::Record, colored: bool) -> String {
    if colored {
        let level = match record.level() {
            Level::Error => Red.paint("ERROR"),
            Level::Warn => Yellow.paint("WARN"),
            Level::Info => Green.paint("INFO"),
            Level::Debug => Purple.paint("DEBUG"),
            Level::Trace => Cyan.paint("TRACE"),
        };

        // https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg
        format!("[{}] {}: {}", Fixed(245).paint(time), level, record.args(),)
    } else {
        format!("[{}] {}: {}", time, record.level(), record.args(),)
    }
}

fn format_hhmmssnnn() -> String {
    let time = Local::now();

    format!(
        "{:02}:{:02}:{:02}.{:03}",
        time.hour(),
        time.minute(),
        time.second(),
        time.nanosecond() / 1_000_000,
    )
}

fn format_yyyymmdd_hhmmss() -> String {
    let time = Local::now();

    format!(
        "{:04}-{:02}-{:02}-{:02}.{:02}.{:02}",
        time.year(),
        time.month(),
        time.day(),
        time.hour(),
        time.minute(),
        time.second()
    )
}
