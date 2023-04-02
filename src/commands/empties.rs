use std::fs::remove_dir_all;
use std::path::{MAIN_SEPARATOR, PathBuf};

use inquire::Confirm;

use crate::commands::walkers::EmptiesWalker;
use crate::utils::{display_error_and_exit, validate_path};

/// Lists empty folders
pub fn list_empties(path: &str, show_hidden: bool) {
    empties_handler(
        "list", path, show_hidden,
        |_, msg| {
            println!("  - {msg}");

            Ok(())
        },
    );
}

/// Removes empty folders
pub fn remove_empties(
    path: &str, confirmed: bool, show_hidden: bool,
) {
    empties_handler(
        "remove", path, show_hidden,
        move |empty, msg| {
            let mut do_it = confirmed;

            if !confirmed {
                let confirmation = Confirm::new(&format!("remove {msg}"))
                    .with_default(false)
                    .with_placeholder("N")
                    .prompt();

                match confirmation {
                    Ok(answer) => do_it = answer,
                    Err(err) => {
                        display_error_and_exit(&format!("Exception processing input: {err}"));
                    }
                }
            };

            if do_it {
                remove_dir_all(empty).map_err(|err| format!("{err}"))?;

                if confirmed { println!("  - {msg} - removed"); }
            }

            Ok(())
        },
    );
}

/// Common empties handling logic
fn empties_handler<F>(
    action: &str, path: &str, show_hidden: bool, handler: F,
)
    where F: Fn(&PathBuf, &str) -> Result<(), String>
{
    validate_path(path);

    let mut found = 0;

    for entry in EmptiesWalker::new(path, show_hidden) {
        let offset = usize::from(!path.ends_with(MAIN_SEPARATOR));
        let output = entry.to_string_lossy()[path.len() + offset..].to_string();

        if let Err(err) = handler(&entry, &output) {
            display_error_and_exit(
                &format!("\nException occurred while {action}ing {output}:\n  {err}")
            );
        }

        found += 1;
    }

    if found == 0 { println!("No empties found at \"{path}\""); }
}
