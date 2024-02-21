use std::collections::HashSet;
use std::path::Path;

use crate::{Platform, Selection, supported_platforms, utils};
use crate::utils::{display_error_and_exit, list_output};

/// Validates a given path exists and it is a folder
pub fn validate_path<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();

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
        let configuration_path = utils::path_of_supported_platforms_configuration();
        let configuration_path = configuration_path.to_string_lossy();

        message.push_str(&format!("\n\nConfigurations file requires manual fix: {configuration_path}"));
    }

    if !message.is_empty() {
        supported_platforms(platforms);
        println!();
        display_error_and_exit(&message);
    }
}

/// Validates all platform filters are supported platforms, case-sensitive
pub fn validate_platforms_filter(filter: &Selection, platforms: &[Platform]) {
    if let Selection::Select { values } = filter {
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

/// Validates all values in a list are unique by it's case insensitive string value
#[inline]
pub fn validate_unique_values<V>(values: &[V]) -> bool
    where V: AsRef<str>
{
    values.iter().map(|v| v.as_ref().to_lowercase()).collect::<HashSet<_>>().len() == values.len()
}
