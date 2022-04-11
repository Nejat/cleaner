use std::fs::remove_dir_all;
use std::path::{MAIN_SEPARATOR, PathBuf};
use std::process::exit;
use std::sync::Once;

use crate::commands::walkers::EmptiesWalker;
use crate::utils::question::{DefaultAnswer, Response, yes_no_question};
use crate::utils::validate_path;

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
            let mut response = Response::Yes { defaulted: true };

            if !confirmed {
                response = yes_no_question(&format!("  - remove {msg}"), DefaultAnswer::No);
            };

            match response {
                Response::Yes { defaulted } => {
                    if !defaulted { println!(); }

                    remove_dir_all(empty).map_err(|err| format!("{err}"))?;

                    if confirmed { println!("  - {msg} - removed"); }
                }
                Response::No { defaulted } if !defaulted => println!(),
                Response::No { .. } => {}
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
    let notify_once = Once::new();

    for entry in EmptiesWalker::new(path, show_hidden) {
        let offset = if path.ends_with(MAIN_SEPARATOR) { 0 } else { 1 };
        let output = entry.to_string_lossy()[path.len() + offset..].to_string();

        notify_once.call_once(|| println!("Found\n"));

        if let Err(err) = handler(&entry, &output) {
            eprintln!("\nException occurred while {action}ing {output}:\n  {err}");
            println!();
            exit(-1);
        }

        found += 1;
    }

    if found == 0 { println!("No empties found at \"{}\"", path); }
}
