use std::process;

use crate::logger::{info, warn};
use crate::logger;
use crate::config::ServerConfig;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
// use std::io::prelude::*;
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Builder;
// use chrono::Utc;
use chrono::Local;

pub fn create_dirs_and_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create directories
    fs::create_dir_all(path.parent().unwrap())?;

    // Create file
    File::create(path)?;

    Ok(())
}

pub fn check_directory_exists(path: &str) -> bool {
    let dir_path = Path::new(path);
    dir_path.is_dir()
}

pub fn create_local_directory(config: &ServerConfig) -> Result<(), std::io::Error> {
    fs::create_dir_all(&config.local_dir)
}

fn create_dir_from_date(parent_dir: &str) -> std::io::Result<()> {
    // let now = Utc::now();
    let now = chrono::offset::Local::now();
    let date = now.format("%Y-%m-%d").to_string();
    let dir_path = Path::new(parent_dir).join(&date);
    fs::create_dir_all(&dir_path)?;
    Ok(())
}

fn add_trailing_slash(path: &str) -> String {
    let mut modified_path = String::from(path);
    if !modified_path.ends_with('/') {
        modified_path.push('/');
    }
    modified_path
}

fn remove_extension(filename: &str) -> &str {
    match filename.rfind('.') {
        Some(index) => &filename[..index],
        None => filename,
    }
}

// let path = "/mnt/backups";
// let modified_path = add_trailing_slash(path);
// println!("{}", modified_path); // Output: "/mnt/backups/"

pub fn create_tarball(dir_to_read: &str, tar_destination: &str, excluded_directories: &[String]) -> io::Result<()> {

    info("Creating date directory");
    let modified_path = add_trailing_slash(tar_destination);
    if check_directory_exists(&modified_path) {
        create_dir_from_date(&modified_path)?;
        if let Err(error) = create_dir_from_date(&modified_path) {
            let error_message = format!("Error creating directory: {}", error);
            logger::error(&error_message);
        }
    } else {
        let error_message = format!("Directory for backups doesn't exist");
        logger::error(&error_message);
    }

    // let date = Utc::today().format("%Y-%m-%d").to_string();
    let date = chrono::offset::Local::today().format("%Y-%m-%d").to_string();

    info("Reading dir");
    for entry in fs::read_dir(dir_to_read)? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                let error_message = format!("Failed to read directory {:?} entry: {}", dir_to_read, error);
                logger::error(&error_message);
                continue;
            }
        };

        let path = entry.path();
        let metadata = match fs::metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) => {
                let error_message = format!("Failed to get metadata for file {:?}: {}", path, error);
                logger::error(&error_message);
                // println!("Failed to get metadata for file: {}", error);
                continue;
            }
        };
        if metadata.is_dir() {
            let original_string = format!("{}", path.display());
            if let Some((_, last_part)) = original_string.rsplit_once('/') {
                let path_to_clean = format!("{}/{}/{}.tar.gz", tar_destination,date,last_part);
                let parts: Vec<&str> = path_to_clean.split('/').filter(|s| !s.is_empty()).collect();
                let tar_name = format!("/{}", parts.join("/"));
                
                let message = format!("Creating tar: {}", tar_name);
                info(&message);
        
                let tar_gz = File::create(&tar_name)?;
                let enc = GzEncoder::new(tar_gz, Compression::default());
                let mut builder = Builder::new(enc);
                let p = path.to_string_lossy().to_string();
                add_dir_to_tar(&mut builder, Path::new(&p), excluded_directories, Path::new(&p))?;
                builder.finish()?;
        
                let message = format!("Tar finished: {}", tar_name);
                info(&message);
                // process::exit(0);
            }
        } else {
            let original_string = format!("{}", path.display());
            if let Some((_, last_part)) = original_string.rsplit_once('/') {
                let last_part = remove_extension(last_part);
                let path_to_clean = format!("{}/{}/{}.tar.gz", tar_destination,date,last_part);
                let parts: Vec<&str> = path_to_clean.split('/').filter(|s| !s.is_empty()).collect();
                let tar_name = format!("/{}", parts.join("/"));
                
                let message = format!("Creating tar: {}", tar_name);
                info(&message);

                let file_path = path.to_string_lossy();
                let transformed_path = if file_path.starts_with('/') {
                    file_path.strip_prefix('/').unwrap_or(&file_path)
                } else {
                    &file_path
                };

                if excluded_directories.iter().any(|s| file_path.contains(&*s)) {
                    let message = format!("Excluded: {:?}", path);
                    warn(&message);
                    continue;
                }

                let tar_gz = File::create(&tar_name)?;
                let enc = GzEncoder::new(tar_gz, Compression::default());
                let mut builder = Builder::new(enc);
                match fs::File::open(&path) {
                    Ok(mut file) => {
                        builder.append_file(transformed_path, &mut file)?;
                    }
                    Err(error) => {
                        let error_message = format!("Failed to open file: {}", error);
                        logger::error(&error_message);
                    }
                }
            }
        }
    }

    Ok(())
}

fn add_dir_to_tar(builder: &mut Builder<GzEncoder<File>>, dir_path: &Path, excluded_directories: &[String], root_path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(dir_path)? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                let error_message = format!("Failed to read directory {:?} entry: {}", dir_path, error);
                logger::error(&error_message);
                // println!("Failed to read directory entry: {}", error);
                continue;
            }
        };

        let path = entry.path();

        // let metadata = fs::metadata(&path)?;
        let metadata = match fs::metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) => {
                let error_message = format!("Failed to get metadata for file {:?}: {}", path, error);
                logger::error(&error_message);
                // println!("Failed to get metadata for file: {}", error);
                continue;
            }
        };

        if metadata.is_dir() {
            let dir_name = path.to_string_lossy();
            if excluded_directories.iter().any(|s| dir_name.contains(&*s)) {
                let message = format!("Excluded: {:?}", path);
                warn(&message);
                // println!("excluded: {:?}", path);
                continue;
            }

            add_dir_to_tar(builder, &path, excluded_directories, &root_path)?;
        } else if metadata.is_file() {
            let file_path = path.to_string_lossy();
            let transformed_path = if file_path.starts_with('/') {
                file_path.strip_prefix('/').unwrap_or(&file_path)
            } else {
                &file_path
            };

            if excluded_directories.iter().any(|s| file_path.contains(&*s)) {
                let message = format!("Excluded: {:?}", path);
                warn(&message);
                // println!("excluded: {:?}", path);
                continue;
            }

            // let mut result = fs::File::open(&path)?;
            match fs::File::open(&path) {
                Ok(mut file) => {
                    builder.append_file(transformed_path, &mut file)?;
                }
                Err(error) => {
                    let error_message = format!("Failed to open file: {}", error);
                    logger::error(&error_message);
                }
            }
            
        }
    }

    Ok(())
}
