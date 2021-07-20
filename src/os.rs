use std::fs;
use std::fs::File;

use std::process::{Command, Stdio};

pub fn get_system(systems: Vec<System>) -> Option<System> {
    let file_contents = fs::read_to_string("/etc/os-release");

    if let Ok(contents) = file_contents {
        for system in systems {
            if contents.contains(system.os_name.as_str()) {
                return Some(system);
            }
        }
    }

    return None;
}

pub fn get_package_managers(managers: &mut Vec<PackageManager>) -> Vec<PackageManager> {
    let to_return: &mut Vec<PackageManager> = &mut Vec::new();

    for manager in managers {
        if let Ok(_ignored) = File::open(manager.file_path.clone()) {
            to_return.push(manager.clone());
        }
    }

    return to_return.clone();
}

pub fn get_packages(managers: Vec<PackageManager>) -> i32 {
    let mut package_amount = 0;

    for manager in managers {
        let command = &mut Command::new(manager.base_command.command);
        let piped_command = &mut Command::new(manager.piped_command.command);

        for arg in manager.base_command.arguments {
            command.arg(arg);
        }

        let result = command.stdout(Stdio::piped()).spawn().unwrap();

        for arg in manager.piped_command.arguments {
            piped_command.arg(arg);
        }

        let output = &piped_command
            .stdin(result.stdout.unwrap())
            .output()
            .expect("");
        let output_str = &mut String::from_utf8_lossy(&output.stdout).to_string();

        output_str.retain(|c| c.is_digit(10));
        package_amount += output_str.parse::<i32>().unwrap();
    }

    return package_amount;
}

pub struct System {
    pub os_name: String,
    pub os_display_name: String,
}

impl System {
    pub fn new(os_name: &str, os_display_name: &str) -> Self {
        Self {
            os_name: os_name.to_string(),
            os_display_name: os_display_name.to_string(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct CommandData {
    pub command: String,
    pub arguments: Vec<String>,
}

impl CommandData {
    pub fn new(command: String, arguments: Vec<String>) -> Self {
        Self {
            command: command,
            arguments: arguments,
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct PackageManager {
    pub display_name: String,
    pub file_path: String,

    pub base_command: CommandData,
    pub piped_command: CommandData,
}

impl PackageManager {
    pub fn new(
        display_name: &str,
        file_path: &str,
        base_command: CommandData,
        piped_command: CommandData,
    ) -> Self {
        Self {
            display_name: display_name.to_string(),
            file_path: file_path.to_string(),
            base_command: base_command,
            piped_command: piped_command,
        }
    }
}
