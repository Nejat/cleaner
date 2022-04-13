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
pub fn reset_configuration() {
    println!("By resetting your configuration you will loose any customization you have applied\n");

    let confirmation = Confirm::new("Are you sure")
        .with_default(false)
        .with_placeholder("N")
        .prompt();

    match confirmation {
        Ok(true) => {
            let path = path_of_supported_platforms_configuration();

            if path.exists() {
                match remove_file(&path) {
                    Ok(_) => {}
                    Err(err) =>
                        display_error_and_exit(&format!("Exception resenting configuration: {err}"))
                }
            }
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


    for platform in platforms {
        if separator { println!(); }

        skip_first.call_once(|| separator = true);

        println!("Platform: {}", platform.name);
        println!("  Build Artifacts: {}", list_output(&platform.folders));
        println!("  Matched On: {}", list_output(&platform.associated));
    }
}

/// Shows the path of the configuration json file
pub fn show_configuration() {
    println!("{}", path_of_supported_platforms_configuration().to_string_lossy());
}