mod config;
pub mod os;

fn main() {
    let system = os::get_system(config::get_systems());

    match system {
        Some(system) => {
            for field in config::get_fields(system) {
                print!("{}", field.format);
            }
        }
        _ => panic!("No supported OS has been found on your system."),
    }
}
