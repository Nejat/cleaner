use std::{env, fs};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Once;

use crate::{AllValues, Platform};

/// Separator for creating a list for string values for display
const SEPARATOR: &str = ", ";

/// loads a configuration of supported platforms
pub fn load_supported_platforms() -> Vec<Platform> {
    const SUPPORTED_PLATFORMS_PATH: &str = "supported-platforms.json";

    let mut path = match env::current_exe() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Exception determining path information: {}", err);
            eprintln!();
            exit(-1);
        }
    };

    path.set_file_name(SUPPORTED_PLATFORMS_PATH);

    if !path.exists() {
        if let Err(err) = fs::write(&path, include_str!("../../supported-platforms.json")) {
            eprintln!("Exception creating configuration file: {}", err);
            eprintln!();
            exit(-1);
        }
    }

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Exception accessing configuration file: {}", err);
            eprintln!();
            exit(-1);
        }
    };

    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(platforms) => platforms,
        Err(err) => {
            // todo: give user option to overwrite corrupt configuration with default configuration
            eprintln!("Exception with configuration: {}", err);
            eprintln!();
            exit(-1);
        }
    }
}

/// creates an easier to read comma separated output from a list
pub fn list_output<T: AsRef<str>>(source: &[T]) -> String {
    let mut output = String::default();
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

/// Validates a given path exists and it is a folder
pub fn validate_path(path: &str) {
    let path = PathBuf::from(path);

    if !path.exists() {
        eprintln!("path: \"{}\" - does not exist!\n", path.to_string_lossy());
        println!();
        exit(-1);
    }

    if path.is_file() {
        eprintln!("path: \"{}\" - is not directory!\n", path.to_string_lossy());
        println!();
        exit(-1);
    }
}

/// Validates al platform filters are supported platforms, case sensitive
pub fn validate_platforms_filter(filter: &AllValues, platforms: &[Platform]) {
    if let AllValues::Values { values } = filter {
        let unsupported = values.iter()
            .filter(|v| platforms.iter().all(|p| &p.name != *v)).collect::<Vec<_>>();

        if !unsupported.is_empty() {
            let pluralized = if unsupported.len() > 1 { "s" } else { "" };

            eprintln!("Unsupported platform{}: {}", pluralized, list_output(&unsupported));
            eprintln!();
            eprintln!("Supported Platforms: {}", list_output(platforms));
            eprintln!();

            exit(-1);
        }
    }
}
