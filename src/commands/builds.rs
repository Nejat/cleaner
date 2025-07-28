use std::fs::remove_dir_all;
use std::path::{Path, MAIN_SEPARATOR};

use crate::commands::walkers::BuildsWalker;
use crate::models::BuildArtifacts;
use crate::utils::{display_error_and_exit, get_confirmation, validate_path, validate_platforms_filter};
use crate::{Platform, Selection};

/// Lists matching build artifacts
pub fn list_build_artifacts<P: AsRef<Path>>(path: P, filter: &Selection, platforms: &[Platform]) {
    build_artifacts_handler(
        "list", path, filter, platforms,
        |_, msg| {
            println!("  - {msg}");

            Ok(())
        },
    );
}

/// Removes matching build artifacts
pub fn remove_build_artifacts<P: AsRef<Path>>(
    path: P, filter: &Selection, platforms: &[Platform], confirmed: bool,
) {
    build_artifacts_handler(
        "remove", path, filter, platforms,
        move |artifact, msg| {
            if confirmed || get_confirmation(&msg) {
                remove_dir_all(&artifact.folder).map_err(|err| format!("{err}"))?;
                if confirmed { println!("  - {msg} - removed"); }
            }

            Ok(())
        },
    );
}

/// Common build artifact handling logic
fn build_artifacts_handler<F, P: AsRef<Path>>(
    action: &str, path: P, filter: &Selection, platforms: &[Platform], handler: F,
)
    where F: Fn(&BuildArtifacts, &str) -> Result<(), String>
{
    validate_path(&path);
    validate_platforms_filter(filter, platforms);

    let path = path.as_ref();
    let path_str = path.to_string_lossy();
    let max_width = platforms.iter().map(|p| p.name.len()).max().unwrap_or_default();
    let mut found = 0;

    for entry in BuildsWalker::new(filter, path, platforms) {
        let offset = usize::from(!path_str.ends_with(MAIN_SEPARATOR));
        let output = format!("[{:max_width$}] {}", entry.name, &entry.folder[path_str.len() + offset..]);

        if let Err(err) = handler(&entry, &output) {
            display_error_and_exit(
                &format!("\nException occurred while {action}ing {output}:\n  {err}")
            );
        }

        found += 1;
    }

    if found == 0 {
        println!(
            "No build artifacts found for {filter}{} platform{}",
            filter.choose("the ", ""), filter.pluralize("s")
        );
    }
}
