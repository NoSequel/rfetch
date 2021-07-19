use crate::os::{self, *};
use colored::*;
use std::{env, process::Command};

pub fn get_systems() -> Vec<System> {
    return vec![
        System::new("Gentoo", "Gentoo Linux"),
        System::new("Arch", "Arch Linux"),
    ];
}

pub fn get_package_managers() -> Vec<PackageManager> {
    return vec![PackageManager::new(
        "pacman",
        "/usr/bin/pacman",
        CommandData::new("pacman".to_string(), vec!["-Qq".to_string()]),
        CommandData::new("wc".to_string(), vec!["-l".to_string()]),
    )];
}

pub fn get_fields(system: System) -> Vec<DataField> {
    return vec![
        DataField::new(format!(
            "{}@{}",
            env::var("USER").unwrap_or_else(|_x| "".to_string()).red(),
            env::var("hostname")
                .unwrap_or_else(|_x| "".to_string())
                .red()
        )),
        DataField::new(format!(
            "{} ~ {}",
            "os".white(),
            system.os_display_name.red()
        )),
        DataField::new(format!(
            "{} ~ {}",
            "sh".white(),
            env::var("SHELL").unwrap_or_else(|_x| "".to_string()).red()
        )),
        DataField::new(format!(
            "{} ~ {}",
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
