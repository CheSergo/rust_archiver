// use crate::config::get_config;
// use crate::mounter::mount;
use crate::dirreader::create_tarball;
use crate::logger::{info, error};
use crate::logger;
use crate::config::Config;
use crate::mounter::umount;

extern crate chrono;
use chrono::prelude::*;
use chrono::{Local, Timelike, Duration, NaiveTime, Weekday};
// use chrono::{Duration, Weekday};
use std::thread::sleep;
use std::time::Duration as StdDuration;

// use std::process;

pub fn num_to_weekday(day: u8) -> Option<Weekday> {
    match day {
        1 => Some(Weekday::Mon),
        2 => Some(Weekday::Tue),
        3 => Some(Weekday::Wed),
        4 => Some(Weekday::Thu),
        5 => Some(Weekday::Fri),
        6 => Some(Weekday::Sat),
        7 => Some(Weekday::Sun),
        _ => None,
    }
}

pub fn run_daily(
    h: u32,
    m: u32,
    on_cron: impl Fn(&Config) -> Result<(), String>,
    days: Vec<u8>,
    config: &Config
) {
    let weekdays: Vec<Weekday> = days.into_iter().filter_map(num_to_weekday).collect();
    println!("{:?}", weekdays);

    loop {
        let now = chrono::offset::Local::now();
        let target_time = now.date().and_hms(h, m, 0);

        let mut sleep_time = if now < target_time {
            target_time - now
        } else {
            target_time + Duration::days(1) - now
        };
        let sleep_time = StdDuration::new(sleep_time.num_seconds() as u64, 0);
        let message = format!("Sleep time first - {:?}", sleep_time);
        info(&message);
        println!("Sleep time first - {:?}", sleep_time);
        sleep(sleep_time);

        if weekdays.contains(&now.date().weekday()) {
            println!("Программа выполняется");
            sleep(StdDuration::new(1, 0));

            match on_cron(&config) {
                Ok(()) => {
                    // Mount operation succeeded
                    let message = format!("Start making tar");
                    info(&message);
                    create_tarball(&config.path, &config.target_folder, &config.excluded_directories)
                        .expect("Failed to parse directory");
                }
                Err(error) => {
                    // Mount operation failed
                    // error("Error: {} ", error);
                    let message = format!("Start making tar");
                    logger::error(&message);
                    println!("Failed to mount directory: {}", error);
                }
            }
            // let _ = mounter::umount(&server_config);
            let _ = umount(&&config.server);
        }
    }
}
