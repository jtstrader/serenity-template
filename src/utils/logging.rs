//! Logging for the bot that supports text logs (for `stdout`) or [GCP Cloud Logging](https://cloud.google.com/logging/docs).
//! Sourced from ralpha's `google_cloud_logging` crate example [here](https://github.com/ralpha/google_cloud_logging/tree/main/examples/log).

#![allow(dead_code)]
use chrono::Utc;
use google_cloud_logging::*;
use log::{Level, Metadata, Record};
use regex::Regex;
use std::{backtrace::Backtrace, env};

/// The log collector and handler for most messages.
pub struct Logger {
    format: LogFormat,
    supported_targets: LogTargets,
}

/// The log format. [`LogFormat::Text`] is for human readable messages, while [`LogFormat::Json`] is
/// for message that will be collected by [GCP Cloud Logging](https://cloud.google.com/logging/docs).
///
/// The desired format will be pulled from the environment using the `LOG_FORMAT` variable. The options
/// are:
///   - `json` => [`LogFormat::Json`]
///   - `text` => [`LogFormat::Text`]
///
/// Note that if any invalid formats are provided in the logger, it will default to using the JSON style
/// to ensure that the production bot is logging properly.
pub enum LogFormat {
    Text,
    Json,
}

impl LogFormat {
    /// Get the log format based on the provided `LOG_FORMAT` environment variable.
    /// The supported options are:
    ///   - `json` => [`LogFormat::Json`]
    ///   - `text` => [`LogFormat::Text`]
    ///
    /// Note that if any invalid formats are provided in the logger, it will default to using the JSON style
    /// to ensure that the production bot is logging properly.
    pub fn get_format() -> Self {
        let fmt = std::env::var("LOG_FORMAT").unwrap_or("json".into());
        match fmt.as_ref() {
            "text" => LogFormat::Text,
            "json" => LogFormat::Json,
            _ => {
                eprintln!("Invalid LOG_FORMAT: {}. Defaulting to JSON.", fmt);
                LogFormat::Json
            }
        }
    }
}

/// A collection of the target regexes for the logger to access. The targets are set
/// in a CSV environment variable `RUST_LOG_TARGETS`. If a regex is matched on the
/// crate of an emitted log, the log will be allowed to pass through.
pub struct LogTargets {
    targets: Vec<Regex>,

    /// Special option to allow all logs through, no matter what targets are provided.
    all: bool,
}

impl Default for LogTargets {
    fn default() -> Self {
        let targets: Vec<_> = env::var("RUST_LOG_TARGETS")
            .map(|csv| csv.split(",").map(String::from).collect())
            .unwrap_or(vec![env!("CARGO_PKG_NAME").to_owned()]);
        let all = targets.iter().any(|target| target.to_lowercase() == "all");

        LogTargets {
            targets: targets
                .into_iter()
                .flat_map(|target| match Regex::new(&target) {
                    Ok(re) => Some(re),
                    Err(e) => {
                        eprintln!(
                            "target regex '{}' failed to compile with error: {}",
                            target, e
                        );
                        None
                    }
                })
                .collect(),
            all,
        }
    }
}

impl Logger {
    pub fn new() -> Self {
        Self {
            format: LogFormat::get_format(),
            supported_targets: LogTargets::default(),
        }
    }

    pub fn custom(format: LogFormat, supported_targets: LogTargets) -> Self {
        Self {
            format,
            supported_targets,
        }
    }

    /// Generate the crate root of a Rust path for logging the operation ID to GCP.
    fn get_crate_root(target: &str) -> &str {
        target.split("::").next().unwrap_or(target)
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.supported_targets.all
            || self
                .supported_targets
                .targets
                .iter()
                .any(|re| re.is_match(metadata.target()))
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();

            match self.format {
                LogFormat::Text => {
                    println!(
                        "{:<5}:{} - {}{}",
                        match level {
                            Level::Error => "ERROR",
                            Level::Warn => "WARN",
                            Level::Info => "INFO",
                            Level::Debug => "DEBUG",
                            Level::Trace => "TRACE",
                        },
                        record.target(),
                        record.args(),
                        match level {
                            Level::Error | Level::Warn => format!("\n{}", Backtrace::capture()),
                            _ => "".to_owned(),
                        }
                    );
                }
                LogFormat::Json => {
                    let log_entry = GoogleCloudStructLog {
                        severity: Some(match level {
                            Level::Error => GCLogSeverity::Error,
                            Level::Warn => GCLogSeverity::Warning,
                            Level::Info => GCLogSeverity::Info,
                            Level::Debug => GCLogSeverity::Debug,
                            Level::Trace => GCLogSeverity::Default,
                        }),
                        report_type: match level {
                            // More info see: https://cloud.google.com/error-reporting/docs/formatting-error-messages#@type
                            Level::Error => Some("type.googleapis.com/google.devtools.clouderrorreporting.v1beta1.ReportedErrorEvent".to_owned()),
                            _ => None,
                        },
                        message: Some(
                            format!(
                                "{}\n{}", 
                                record.args(),
                                Backtrace::capture(),
                            )
                        ),
                        operation: Some(GCOperation {
                            id: Some(Logger::get_crate_root(record.metadata().target())),
                            producer: Some(record.metadata().target()),
                            ..Default::default()
                        }),
                        source_location: Some(GCSourceLocation {
                            file: record.file_static(),
                            line: record.line().map(|s| s.to_string()),
                            function: record.module_path_static(),
                        }),
                        time: Some(Utc::now()),
                        ..Default::default()
                    };
                    println!(
                        "{}",
                        serde_json::to_string(&log_entry).expect("Error during logging")
                    );
                }
            }
        }
    }

    fn flush(&self) {}
}

/// Create a logger with the specified format set in the `LOG_FORMAT` environment variable.
/// See [`LogFormat`] for more details.
pub(crate) fn setup_logger() -> anyhow::Result<()> {
    log::set_boxed_logger(Box::new(Logger::new()))?;
    if cfg!(debug_assertions) {
        log::set_max_level(log::LevelFilter::Trace);
    } else {
        log::set_max_level(log::LevelFilter::Info);
    }

    Ok(())
}
