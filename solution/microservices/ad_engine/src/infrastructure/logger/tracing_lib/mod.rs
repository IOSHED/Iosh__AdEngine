use std::{fs::File, path::PathBuf};

use chrono::Timelike;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

use crate::infrastructure;

async fn generate_name_for_file(path_to_file: PathBuf, file_name: &str) -> PathBuf {
    let current_time = chrono::Utc::now();
    let formatted_timestamp = format!(
        "{}-{:02}-{:02}-{:02}",
        current_time.format("%Y-%m-%d"),
        current_time.hour(),
        current_time.minute(),
        current_time.second()
    );

    let log_file_name = format!("{}-{}.log", file_name, formatted_timestamp);
    path_to_file.join(log_file_name)
}

/// Loggers: `console`, `file` for all logs, `file` for error logs.
pub async fn setup_logging(config: infrastructure::configurate::LoggerConfig) {
    let user_level_file_writer =
        File::create(generate_name_for_file(config.log_dir.clone(), "all").await).expect("Failed to create log file");
    let error_level_file_writer =
        File::create(generate_name_for_file(config.log_dir, "error").await).expect("Failed to create log file");

    let formatting_layer = tracing_subscriber::fmt::layer().pretty();

    tracing_subscriber::Registry::default()
        .with(formatting_layer)
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_ansi(false)
                .with_writer(error_level_file_writer)
                .with_filter(config.max_level_error_file),
        )
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_ansi(false)
                .with_writer(user_level_file_writer)
                .with_filter(config.max_level_file),
        )
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_writer(std::io::stdout)
                .with_ansi(true),
        )
        .with(config.max_level_cmd)
        .init();
}
