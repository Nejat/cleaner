use std::fs::remove_dir_all;
use std::path::MAIN_SEPARATOR;
use std::process::exit;

use inquire::Confirm;

use crate::{AllValues, Platform};
use crate::commands::walkers::BuildsWalker;
use crate::models::BuildArtifacts;
use crate::utils::{validate_path, validate_platforms_filter};

/// Lists matching build artifacts
pub fn list_build_artifacts(path: &str, filter: &AllValues, platforms: &[Platform]) {
    build_artifacts_handler(
        "list", path, filter, platforms,
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
        "remove", path, filter, platforms,
        move |artifact, msg| {
            let mut do_it = confirmed;

            if !confirmed {
                let confirmation = Confirm::new(&format!("remove {msg}"))
                    .with_default(false)
                    .with_placeholder("N")
                    .prompt();

                match confirmation {
                    Ok(answer) => do_it = answer,
                    Err(err) => {
                        eprintln!("Exception processing input: {}", err);
                        eprintln!();
                        exit(-1);
                    }
                }
            };

            if do_it {
                remove_dir_all(&artifact.folder).map_err(|err| format!("{err}"))?;
                if confirmed { println!("  - {msg} - removed"); }
            }

            Ok(())
        },
    );
}

/// Common build artifact handling logic
fn build_artifacts_handler<F>(
    action: &str, path: &str, filter: &AllValues, platforms: &[Platform], handler: F,
)
    where F: Fn(&BuildArtifacts, &str) -> Result<(), String>
{
    validate_path(path);
    validate_platforms_filter(filter, platforms);

    let max_width = platforms.iter().map(|p| p.name.len()).max().unwrap_or_default();
    let mut found = 0;

    for entry in BuildsWalker::new(filter, path, platforms) {
        let offset = if path.ends_with(MAIN_SEPARATOR) { 0 } else { 1 };
        let output = format!("[{:max$}] {}", entry.name, &entry.folder[path.len() + offset..], max = max_width);

        if let Err(err) = handler(&entry, &output) {
            eprintln!("\nException occurred while {action}ing {output}:\n  {err}");
            println!();
            exit(-1);
        }

        found += 1;
    }

    if found == 0 {
        // todo: "the all platform(s)" is not grammatically correct
        // todo: add an & for the last listed platform, if more than one platform
        println!("No build artifacts found for the {} platform{}", filter, filter.pluralize("s"));
    }
}
