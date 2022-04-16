use std::{env, fs};
use std::collections::HashSet;
use std::fs::{File, remove_file};
use std::io::BufReader;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Once;

use inquire::Confirm;

use crate::{AllValues, Platform, supported_platforms};

/// Separator for creating a list for string values for display
const SEPARATOR: &str = ", ";

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

/// Creates an easier to read comma separated output from a list
pub fn list_output<T: AsRef<str>>(source: &[T]) -> String {
    let mut output = String::default();

    if source.is_empty() {
        return output;
    }

    let mut add_separator = false;
    let skip_first = Once::new();

    for item in source.iter().take(source.len() - 1) {
        if add_separator { output.push_str(SEPARATOR); }

        skip_first.call_once(|| add_separator = true);

        output.push_str(item.as_ref());
    }

    if let Some(last) = source.last() {
        if !output.is_empty() {
            output.push_str(" & ");
        }

        output.push_str(last.as_ref());
    }


    output
}

/// Gets the path of the supported platforms configuration json file
pub fn path_of_supported_platforms_configuration() -> PathBuf {
    const SUPPORTED_PLATFORMS_PATH: &str = "supported-platforms.json";

    let mut path = match env::current_exe() {
        Ok(path) => path,
        Err(err) => {
            display_error_and_exit(&format!("Exception determining path information: {err}"));
        }
    };

    path.set_file_name(SUPPORTED_PLATFORMS_PATH);

    path
}

/// Validates a given path exists and it is a folder
pub fn validate_path(path: &str) {
    let path = PathBuf::from(path);

    if !path.exists() {
        display_error_and_exit(
            &format!("path: \"{}\" - does not exist!\n", path.to_string_lossy())
        );
    }

    if path.is_file() {
        display_error_and_exit(
            &format!("path: \"{}\" - is not directory!\n", path.to_string_lossy())
        );
    }
}

/// Validates a single platform
pub fn validate_platform(platform: &Platform) -> bool {
    !platform.name.contains(' ') && !platform.folders.is_empty()
}

/// Validates all platforms
pub fn validate_platforms(platforms: &[Platform]) {
    let has_platforms_with_spaces = platforms.iter().any(|p| p.name.contains(' '));
    let not_unique_names = platforms.iter()
        .map(|p| p.name.to_lowercase())
        .collect::<HashSet<_>>()
        .len() != platforms.len();

    let mut message = String::default();

    if not_unique_names || has_platforms_with_spaces {
        message.push_str("* Platform names ");

        if has_platforms_with_spaces {
            message.push_str("can not contain spaces");
        }

        if not_unique_names {
            if message.ends_with('s') {
                message.push_str(" and ");
            }

            message.push_str("must be unique");
        }
    }

    let no_builds = platforms.iter().any(|p| p.folders.is_empty());
    let builds_not_unique = platforms.iter().any(|p| !validate_unique_values(&p.folders));

    if no_builds || builds_not_unique {
        if !message.is_empty() { message.push('\n'); }

        message.push_str("* Platform build artifacts ");

        if no_builds {
            message.push_str("require at lease one value");
        }

        if builds_not_unique {
            if no_builds {
                message.push_str(" and ");
            }
            message.push_str("must be unique");
        }
    }

    let associated_not_unique = platforms.iter().any(|p| !validate_unique_values(&p.associated));

    if associated_not_unique {
        if !message.is_empty() { message.push('\n'); }

        message.push_str("* Platform associated files and folders must be unique");
    }

    if !message.is_empty() {
        let configuration_path = path_of_supported_platforms_configuration();
        let configuration_path = configuration_path.to_string_lossy();

        message.push_str(&format!("\n\nConfigurations file requires manually fix: {}", configuration_path));
    }

    if !message.is_empty() {
        supported_platforms(platforms);
        println!();
        display_error_and_exit(&message);
    }
}

/// Validates all platform filters are supported platforms, case sensitive
pub fn validate_platforms_filter(filter: &AllValues, platforms: &[Platform]) {
    if let AllValues::Values { values } = filter {
        let unsupported = values.iter()
            .filter(|v| platforms.iter().all(|p| p.name.eq_ignore_ascii_case(v))).collect::<Vec<_>>();

        if !unsupported.is_empty() {
            let pluralized = if unsupported.len() > 1 { "s" } else { "" };

            display_error_and_exit(&format!(
                "Unsupported platform{}: {}\nSupported Platforms: {}",
                pluralized, list_output(&unsupported), list_output(platforms)
            ));
        }
    }
}

#[inline]
pub fn display_error_and_exit(message: &str) -> ! {
    eprintln!("\n{}", message);
    eprintln!();

    exit(-1);
}

#[inline]
pub fn validate_unique_values<V>(values: &[V]) -> bool
    where V: AsRef<str>
{
    values.iter().map(|v| v.as_ref().to_lowercase()).collect::<HashSet<_>>().len() == values.len()
}