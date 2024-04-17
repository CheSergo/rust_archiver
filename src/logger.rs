use crate::config::get_config;
use crate::dirreader::create_dirs_and_file;

// use std::fs::File;
use std::fs::OpenOptions;

use std::path::{Path, PathBuf};
// use std::process;
use std::io::{Write};
// use chrono::Utc;
use chrono::Local;

fn format_log_message(log_level: &str, message: &str) -> String {
    // let timestamp = Utc::now().to_rfc3339();
    let timestamp = chrono::offset::Local::now().to_rfc3339();
    format!("{} [{}] {}", timestamp, log_level, message)
}

fn write_log(log_message: &str) {
    let config_file_path = Path::new("./config/config.yml");
    let log_file_path = Path::new("./initial_status.log");

    let mut path_ref: PathBuf = PathBuf::new();

    if config_file_path.exists() {
        let config = get_config();
        let path_to_log = Path::new(&config.log_path);
        if !path_to_log.exists() {
            if let Err(err) = create_dirs_and_file(&path_to_log) {
                eprintln!("Failed to create directories and file: {}", err);
                return;
            }
        }

        path_ref = path_to_log.to_path_buf();
    } else if !log_file_path.exists() {
        // Create the file if it doesn't exist
        std::fs::File::create(&log_file_path)
        .expect("Could not create file");

        path_ref = log_file_path.to_path_buf();
    } else if log_file_path.exists() && !config_file_path.exists() {
        path_ref = log_file_path.to_path_buf();
    }

    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    // .open("./status.log") // ZDES BYDET PATH
    .open(path_ref)
    .expect("Could not open file");
    writeln!(file, "{}", log_message).expect("Could not write to file");
    // process::exit(0);
}

pub fn info(message: &str) {
    let log_level = "INFO";
    let log_message = format_log_message(log_level, message);

    let writer = write_log(&log_message);
}

pub fn warn(message: &str) {
    let log_level = "WARN";
    let log_message = format_log_message(log_level, message);

    let writer = write_log(&log_message);
}

pub fn error(message: &str) {
    let log_level = "ERROR";
    let log_message = format_log_message(log_level, message);

    let writer = write_log(&log_message);
}