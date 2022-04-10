use std::path::PathBuf;
use std::process::exit;

use crate::{AllValues, Platform};

pub mod question;

/// Separator for creating a list for string values for display
pub const SEPARATOR: &str = ", ";

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
        let unsupported = values.iter().filter_map(
            |v| if platforms.iter().all(|p| p.name != v) {
                Some(v.to_string())
            } else {
                None
            }
        ).collect::<Vec<_>>();

        if !unsupported.is_empty() {
            let pluralized = if unsupported.len() > 1 { "s" } else { "" };

            eprintln!(
                "Unsupported platform{}: {}", pluralized,
                unsupported.join(SEPARATOR)
            );
            eprintln!(
                "Supported Platforms: {}",
                platforms.iter().map(|p| p.name.to_string()).collect::<Vec<_>>().join(SEPARATOR)
            );
            println!();

            exit(-1);
        }
    }
}
