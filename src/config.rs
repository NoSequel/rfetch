use crate::os::{self, *};

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
        DataField::new(format!("{} ~ {}", "os", system.os_display_name)),
        DataField::new(format!(
            "{} ~ {}",
            "pkgs",
            os::get_packages(os::get_package_managers(&mut get_package_managers()))
        )),
    ];
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
