use crate::os::{self, *};
use colored::*;
use std::{env, process::Command};

pub fn get_systems() -> Vec<System> {
    return vec![
        System::new("Gentoo", "Gentoo Linux"),
        System::new("Arch", "Arch Linux"),
        System::new("Void", "Void Linux"),
        System::new("Debian", "Debian Linux"),
    ];
}

pub fn get_package_managers() -> Vec<PackageManager> {
    return vec![
        PackageManager::new(
            "pacman",
            "/usr/bin/pacman",
            CommandData::new("pacman".to_string(), vec!["-Qq".to_string()]),
            CommandData::new("wc".to_string(), vec!["-l".to_string()]),
        ),
        PackageManager::new(
            "xbps",
            "/usr/bin/xbps-install",
            CommandData::new("xbps-query".to_string(), vec!["-l".to_string()]),
            CommandData::new("wc".to_string(), vec!["-l".to_string()]),
        ),
        PackageManager::new(
            "dpkg",
            "/usr/bin/dpkg-query",
            CommandData::new(
                "dpkg-query".to_string(),
                vec!["-f".to_string(), "'.\n'".to_string(), "-W".to_string()],
            ),
            CommandData::new("wc".to_string(), vec!["-l".to_string()]),
        ),
    ];
}

pub fn get_fields(system: System) -> Vec<DataField> {
    return vec![
        DataField::new(format!(
            "{}@{}\n",
            env::var("USER").unwrap_or_else(|_x| "".to_string()).red(),
            env::var("hostname")
                .unwrap_or_else(|_x| "".to_string())
                .red()
        )),
        DataField::new(format!(
            "{} ~ {}\n",
            "os".white(),
            system.os_display_name.red()
        )),
        DataField::new(format!(
            "{} ~ {}\n",
            "sh".white(),
            env::var("SHELL").unwrap_or_else(|_x| "".to_string()).red()
        )),
        DataField::new(format!(
            "{} ~ {}\n",
            "pkgs".white(),
            os::get_packages(os::get_package_managers(&mut get_package_managers()))
                .to_string()
                .red()
        )),
        DataField::new(format!(
            "{} ~ {}",
            "kernel".white(),
            get_kernel_version().red()
        )),
    ];
}

fn get_kernel_version() -> String {
    let command = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Unable to execute command");

    return String::from_utf8_lossy(&command.stdout).to_string();
}

pub struct DataField {
    pub format: String,
}

impl DataField {
    pub fn new(format: String) -> Self {
        Self {
            format: format.to_string(),
        }
    }
}
