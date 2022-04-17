use std::env::current_exe;
use std::fs::{File, remove_file};
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

use inquire::Confirm;

use crate::Platform;
use crate::utils::{display_error_and_exit, validate_platforms};

/// Loads a configuration of supported platforms
pub fn load_supported_platforms() -> Vec<Platform> {
    let path = path_of_supported_platforms_configuration();
    let mut retry = false;

    loop {
        if retry && path.exists() {
            if let Err(err) = remove_file(&path) {
                display_error_and_exit(&format!("Exception resetting configuration: {err}"));
            }
        }

        if !path.exists() {
            if let Err(err) = fs::write(&path, include_str!("../../supported-platforms.json")) {
                display_error_and_exit(&format!("Exception creating configuration file: {err}"));
            }
        }

        let file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => {
                display_error_and_exit(&format!("Exception accessing configuration file: {err}"));
            }
        };

        let reader = BufReader::new(file);
        let exception_message = |err| format!("Exception with configuration: {err}");

        let platforms: serde_json::Result<Vec<Platform>> = serde_json::from_reader(reader);

        match platforms {
            Ok(platforms) => {
                validate_platforms(&platforms);
                return platforms;
            }
            Err(err) if retry =>
                display_error_and_exit(&exception_message(err)),
            Err(err) => {
                let message = exception_message(err);

                eprintln!("{}\n", message);

                let confirmation = Confirm::new("Would you like to reset it, prior changes will be lost")
                    .with_default(false)
                    .with_placeholder("N")
                    .prompt();

                match confirmation {
                    Ok(true) => retry = true,
                    _ => display_error_and_exit(&message)
                }
            }
        }
    }
}


/// Gets the path of the supported platforms configuration json file
pub fn path_of_supported_platforms_configuration() -> PathBuf {
    const SUPPORTED_PLATFORMS_PATH: &str = "supported-platforms.json";

    let mut path = match current_exe() {
        Ok(path) => path,
        Err(err) => {
            display_error_and_exit(&format!("Exception determining path information: {err}"));
        }
    };

    path.set_file_name(SUPPORTED_PLATFORMS_PATH);

    path
}
