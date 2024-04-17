mod config;
mod dirreader;
mod mounter;
mod logger;
mod scheduler;

use crate::mounter::{mount, umount};

use chrono::prelude::*;

fn main() {
    logger::info("Program started");
    config::check_create_config_file();

    logger::info("Initializing config");
    let config = config::get_config();
    let hour: Result<u32, _> = config.duration.hour.parse();
    let minute: Result<u32, _> = config.duration.minute.parse();

    let vec = config::string_to_vec(&config.duration.day_week);
    println!("{:?}", vec);
    
    match (hour, minute) {
        (Ok(h), Ok(m)) => {
            logger::info("Program execution started");

            // let specific_time = NaiveTime::from_hms(h, m, 0);

            // let current_time = chrono::offset::Local::now();
            // println!("{}", current_time.date().weekday());
            scheduler::run_daily(h, m, |config| mount(&&config.server), vec, &config);
        }
        (Err(e), _) => {
            println!("Failed to parse hour: {}", e);
            let message = format!("Failed to parse hour: {}", e);
            logger::error(&message);
        }
        (_, Err(e)) => {
            println!("Failed to parse minute: {}", e);
            let message = format!("Failed to parse minute: {}", e);
            logger::error(&message);
        }
    }
    let _ = umount(&&config.server);
    let message = format!("Unmount the dir: {}", config.server.local_dir);
    logger::info(&message);
    logger::info("Program execution completed");
    logger::info("Shutting down...");
    logger::info("Program ended");
}
