use std::collections::HashSet;
use std::fs::remove_dir_all;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

use crate::commands::walkers::EmptiesWalker;
use crate::models::Platform;
use crate::utils::{display_error_and_exit, get_confirmation, validate_path};

/// Lists empty folders
pub fn list_empties<P: AsRef<Path>>(path: P, show_hidden: bool, platforms: &[Platform]) {
    empties_handler("list", path, show_hidden, platforms, |_, msg| {
        println!("  - {msg}");

        Ok(())
    });
}

/// Removes empty folders
pub fn remove_empties<P: AsRef<Path>>(
    path: P, confirmed: bool, show_hidden: bool, platforms: &[Platform],
) {
    empties_handler("remove", path, show_hidden, platforms, move |empty, msg| {
        if confirmed || get_confirmation(&msg) {
            remove_dir_all(empty).map_err(|err| format!("{err}"))?;

            if confirmed {
                println!("  - {msg} - removed");
            }
        }

        Ok(())
    });
}

/// Common empties handling logic
fn empties_handler<F, P: AsRef<Path>>(
    action: &str, path: P, show_hidden: bool, platforms: &[Platform], handler: F,
)
where
    F: Fn(&PathBuf, &str) -> Result<(), String>,
{
    let path = path.as_ref();
    let path_str = path.to_string_lossy();

    validate_path(path);

    let mut found = 0;
    let skipped = platforms.iter()
        .flat_map(|p| p.folders.iter().map(String::as_str))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    for entry in EmptiesWalker::new(path, show_hidden, &skipped) {
        let offset = usize::from(!path_str.ends_with(MAIN_SEPARATOR));
        let output = entry.to_string_lossy()[path_str.len() + offset..].to_string();

        if let Err(err) = handler(&entry, &output) {
            display_error_and_exit(&format!(
                "\nException occurred while {action}ing {output}:\n  {err}"
            ));
        }

        found += 1;
    }

    if found == 0 {
        println!("No empties found at \"{path_str}\"");
    }
}
