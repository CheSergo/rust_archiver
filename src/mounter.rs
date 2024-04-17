extern crate sys_mount;
use crate::config::ServerConfig;
use crate::logger::{info, warn, error};

use std::process::Command;
use std::path::Path;
use sys_mount::{unmount, UnmountFlags};

fn is_mount_point(path: &str) -> Result<bool, std::io::Error> {
    let path = Path::new(path);

    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "The path doesn't exist"
        ));
    }

    let output = Command::new("mount")
        .arg("-l")
        .output()?;

    let output = String::from_utf8(output.stdout).unwrap();
    Ok(output.contains(path.to_str().unwrap()))
}

pub fn mount(config: &&ServerConfig) -> Result<(), String> {

    println!("{:?}", config);

    match is_mount_point(&config.local_dir) {
        Ok(is_mount) => {
            if is_mount {
                // println!("{} is a mount point", config.local_dir);
                let message = format!("{} is a mount point", config.local_dir);
                info(&message);
                return Ok(());
            } else {
                // println!("{} is not a mount point", config.local_dir);
                let message = format!("{} is not a mount point", config.local_dir);
                warn(&message);

                let mount_command = format!(
                    "echo {} | sshfs {}@{}:{} {} -o password_stdin",
                    config.password, config.username, config.server_ip, config.remote_dir, config.local_dir,
                );

                let output = Command::new("sh")
                    .arg("-c")
                    .arg(&mount_command)
                    .output()
                    .map_err(|_| "Failed to execute mount command".to_string())?;

                if output.status.success() {
                    // println!("Directory mounted successfully");
                    info("Directory mounted successfully");

                    Ok(())
                } else {
                    let error_message = String::from_utf8_lossy(&output.stderr);
                    // println!("Failed to mount directory: {}", error_message);
                    let message = format!("Failed to mount directory: {}", error_message);
                    error(&message);
                    Err(format!("Failed to mount directory: {}", error_message))
                }
            }
        },
        Err(e) => {
            let error_message = format!("Failed to check if {} is mount point: {}", config.local_dir, e);
            error(&error_message);
            // eprintln!("Failed to check if {} is mount point: {}", config.local_dir, e);
            Err(format!("Failed to check if {} is mount point: {}", config.local_dir, e))
        }
    }

}

pub fn umount(config: &&ServerConfig) -> Result<(), String> {
    // Unmount device at `/target/path` lazily.
    let path = &config.local_dir;
    match unmount(path, UnmountFlags::DETACH) {
        Ok(_) => {
            // println!("Successfully unmounted {}", path);
            let message = format!("Successfully unmounted {}", path);
            info(&message);
            Ok(())
        }
        Err(err) => {
            let error_message = format!("Failed to unmount {}: {}", path, err);
            error(&error_message);
            // eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}
