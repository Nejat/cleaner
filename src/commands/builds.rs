use std::fs::remove_dir_all;
use std::process::exit;
use std::sync::Once;

use crate::{AllValues, Platform};
use crate::commands::walker::BuildsWalker;
use crate::models::BuildArtifacts;
use crate::utils::{validate_path, validate_platforms_filter};
use crate::utils::question::{DefaultAnswer, Response, yes_no_question};

/// Lists matching build artifacts
pub fn list_build_artifacts(path: &str, filter: &AllValues, platforms: &[Platform]) {
    build_artifacts_handler(
        "list", &path, filter, platforms,
        |_, msg| {
            println!("  - {msg}");

            Ok(())
        },
    );
}

/// Removes matching build artifacts
pub fn remove_build_artifacts(
    path: &str, filter: &AllValues, platforms: &[Platform], confirmed: bool,
) {
    build_artifacts_handler(
        "remove", &path, filter, platforms,
        move |artifact, msg| {
            let mut response = Response::Yes { defaulted: true };

            if !confirmed {
                response = yes_no_question(&format!("  - remove {msg}"), DefaultAnswer::No);
            };

            match response {
                Response::Yes { defaulted } => {
                    if !defaulted {
                        println!();
                    }

                    remove_dir_all(&artifact.folder).map_err(|err| format!("{err}"))?;

                    if confirmed {
                        println!("  - {msg} - removed");
                    }
                }
                Response::No { defaulted } if !defaulted =>
                    println!(),
                Response::No { .. } => {}
            }

            Ok(())
        },
    );
}

/// Common build artifact handling logic
fn build_artifacts_handler<F>(
    action: &str, path: &&str, filter: &AllValues, platforms: &[Platform], handler: F,
)
    where F: Fn(&BuildArtifacts, &str) -> Result<(), String>
{
    validate_path(path);
    validate_platforms_filter(filter, platforms);

    let max_width = platforms.iter().map(|p| p.name.len()).max().unwrap_or_default();
    let mut found = 0;
    let notify_once = Once::new();

    for entry in BuildsWalker::new(filter, path, platforms) {
        let output = format!("{:max$} > {}", entry.name, &entry.folder[path.len() + 1..], max = max_width);

        notify_once.call_once(|| println!("Found\n"));

        if let Err(err) = handler(&entry, &output) {
            eprintln!("\nException occurred while {action}ing {output}:\n  {err}");
            println!();
            exit(-1);
        }

        found += 1;
    }

    if found == 0 {
        println!("No build artifacts found for the {} platform{}", filter, filter.pluralize("s"));
    }
}
