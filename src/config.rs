use crate::logger::{info, warn,error};
use crate::dirreader::create_dirs_and_file;

use serde::{Serialize, Deserialize};
use serde_yaml::{self};

use std::path::Path;
use std::fs;
use std::process;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub path: String,
    pub target_folder: String,
    pub excluded_directories: Vec<String>,
    pub log_path: String,
    pub duration: DurationConfig,

    pub server: ServerConfig,
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DurationConfig {
    pub day_week: String,
    pub month: String,
    pub day: String,
    pub hour: String,
    pub minute: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub server_ip: String,
    pub username: String,
    pub password: String,
    pub remote_dir: String,
    pub local_dir: String,
}

pub fn get_config()->Config {
    let f = std::fs::File::open("./config/config.yml").expect("Could not open file.");
    let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    scrape_config
}

pub fn check_create_config_file() {
    // let config_dir = Path::new("./config");
    let config_path = Path::new("./config/config.yml");

    let mut config_created = false;

    if !config_path.exists() {
        let message = format!("Config file doesn't exist");
        warn(&message);
        let message = format!("Config file would be created at {:?}", config_path);
        warn(&message);
        let message = format!("Now you can change the log file path. Also default config file created. You can find it at - {:?}", config_path);
        warn(&message);
        let example_config = r#"
# Example main config
path: /var/www
target_folder: /mnt/backups/
duration:
  day_week: 1
  month: 1
  day: "*"
  hour: 12
  minute: 00
excluded_directories:
  - /var/www/r_archiver/target
  - /var/www/rustlings
  - /var/www/for_study/.git/

server:
  server_ip: "172.16.2.01"
  username: "user"
  password: "password"
  remote_dir: "/backup/dir_for_backup"
  local_dir: "/mnt/backups"

log_path: "./status.log"
"#;

        if let Err(err) = create_dirs_and_file(&config_path) {
          eprintln!("Failed to create directories and file: {}", err);
          return;
        }

        match fs::write(config_path, example_config) {
            Ok(_) => {
                let message = format!("Example config file created at {:?}", config_path);
                info(&message);
                config_created = true;
            }
            Err(err) => {
                let message = format!("Failed to create config file: {}", err);
                error(&message);
                process::exit(1);
            }
        }

    } else {
        let message = format!("Config file already exists at {:?}", config_path);
        info(&message);
    }

}

pub fn string_to_vec(input: &String) -> Vec<u8> {
    input.split(", ")
        .map(|num| num.parse().unwrap())
        .collect()
}
