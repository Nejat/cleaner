use std::collections::{HashMap, HashSet};
use std::fs::remove_file;
use std::sync::Once;

use inquire::Confirm;

use crate::Platform;
use crate::utils::{display_error_and_exit, list_output, path_of_supported_platforms_configuration};

/// Manage supported platforms configuration
pub fn manage_configuration() {
    println!("not yet implemented");
}

/// Deletes platform configuration file to reset configuration to default
pub fn reset_configuration(confirmed: bool) {
    if !path_of_supported_platforms_configuration().exists() {
        println!("Configuration of supported platforms is reset");
        return;
    }

    if confirmed {
        reset_configuration_json();
        return;
    }

    println!("By resetting your configuration you will loose any customization you have applied\n");

    let confirmation = Confirm::new("Are you sure")
        .with_default(false)
        .with_placeholder("N")
        .prompt();

    match confirmation {
        Ok(true) => {
            println!();
            reset_configuration_json();
        }
        Ok(false) => {}
        Err(err) =>
            display_error_and_exit(&format!("Exception confirming reset: {err}"))
    }
}

/// List supported platform configuration
pub fn supported_platforms(platforms: &[Platform]) {
    let mut separator = false;
    let skip_first = Once::new();
    let dupes = platforms.iter().fold(
        HashMap::new(),
        |mut acc, next| {
            let entry = acc.entry(next.name.to_lowercase()).or_insert(0);

            *entry += 1;

            acc
        },
    )
        .into_iter()
        .filter_map(|(key, count)| { if count > 1 { Some(key) } else { None } })
        .collect::<HashSet<_>>();

    let status = |name: &str| if dupes.contains(&name.to_lowercase()) {
        " <<= duplicate platform name"
    } else if name.contains(' ') {
        " <<= name contains space(s)"
    } else {
        ""
    };

    for platform in platforms {
        if separator { println!(); }

        skip_first.call_once(|| separator = true);

        println!("Platform: {}{}", platform.name, status(&platform.name));
        println!("  Build Artifacts: {}", list_output(&platform.folders));
        println!("  Matched On: {}", list_output(&platform.associated));
    }
}

/// Shows the path of the configuration json file
pub fn show_configuration() {
    println!("{}", path_of_supported_platforms_configuration().to_string_lossy());
}

/// Deletes supported platforms configuration file
fn reset_configuration_json() {
    let path = path_of_supported_platforms_configuration();

    if path.exists() {
        match remove_file(&path) {
            Ok(_) => println!("Configuration of supported platforms has been reset"),
            Err(err) =>
                display_error_and_exit(&format!("Exception resenting configuration: {err}"))
        }
    }
}
