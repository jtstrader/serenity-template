//! Logging for the bot that supports text logs (for `stdout`) or [GCP Cloud Logging](https://cloud.google.com/logging/docs).
//! Sourced from ralpha's `google_cloud_logging` crate example [here](https://github.com/ralpha/google_cloud_logging/tree/main/examples/log).

#![allow(dead_code)]
use chrono::Utc;
use google_cloud_logging::*;
use log::{Level, Metadata, Record};
use std::backtrace::Backtrace;

/// The log collector and handler for most messages.
pub struct Logger {
    format: LogFormat,
}

/// The log format. [`LogFormat::Text`] is for human readable messages, while [`LogFormat::Json`] is
/// for message that will be collected by [GCP Cloud Logging](https://cloud.google.com/logging/docs).
pub enum LogFormat {
    Text,
    Json,
}

impl LogFormat {
    pub fn get_format() -> Self {
        let log_format = std::env::var("LOG_FORMAT").unwrap_or_else(|_| "text".to_owned());
        match log_format.as_ref() {
            "json" => LogFormat::Json,
            _ => LogFormat::Text,
        }
    }
}

impl Logger {
    pub fn new() -> Self {
        Self {
            format: LogFormat::get_format(),
        }
    }

    pub fn custom(format: LogFormat) -> Self {
        Self { format }
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // We just want everything in this example
        true
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
                            Level::Error | Level::Warn => Backtrace::capture().to_string(),
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
                                "{}{}", 
                                record.args(),
                                Backtrace::capture(),
                            )
                        ),
                        operation: Some(GCOperation {
                            id: Some("My Service"),
                            producer: Some("MyService.Backend"),
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
