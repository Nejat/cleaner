use std::collections::{HashMap, HashSet};
use std::fs::{OpenOptions, remove_file};
use std::io::BufWriter;
use std::sync::Once;

use inquire::{Confirm, MultiSelect, Select, Text};
use inquire::validator::StringValidator;

use crate::{Platform, PLATFORMS};
use crate::models::Filter;
use crate::utils::{display_error_and_exit, list_output, path_of_supported_platforms_configuration, validate_platform, validate_unique_values};

/// Manage supported platforms configuration
pub fn manage_configuration() {
    const ADD: &str = "Add";
    const MODIFY: &str = "Modify";
    const DELETE: &str = "Delete";
    const SAVE: &str = "Save & Exit";
    const CANCEL: &str = "Cancel Modifications";
    const LIST: &str = "List";

    let mut platforms = PLATFORMS.clone();
    let mut modified = false;

    loop {
        let commands = if modified {
            vec![CANCEL, SAVE, ADD, DELETE, MODIFY, LIST]
        } else {
            vec![ADD, DELETE, MODIFY, SAVE, CANCEL, LIST]
        };

        let selection = Select::new("Pick an action", commands).prompt();

        match selection {
            Ok(ADD) => modified = add_new_platform(&mut platforms) || modified,
            Ok(DELETE) => modified = delete_platforms(&mut platforms) || modified,
            Ok(MODIFY) => modified = modify_a_platform(&mut platforms) || modified,
            Ok(SAVE) => if !modified || save_platforms(&platforms) { break; },
            Ok(CANCEL) => if !modified || confirmed("Unsaved changes will be lost") { break; },
            Ok(LIST) => {
                println!();
                supported_platforms(&platforms);
                println!();
            }
            Err(_) => break,
            Ok(_) => unreachable!(),
        }
    }
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

    if super::supported::confirmed(
        "By resetting your configuration you will loose any customization you have applied"
    ) {
        println!();
        reset_configuration_json();
    }
}

/// Lists supported platform configuration
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

    let check_name = |name: &str| if dupes.contains(&name.to_lowercase()) {
        " <<= duplicate platform name"
    } else if name.contains(' ') {
        " <<= name contains space(s)"
    } else {
        ""
    };

    let check_builds = |builds: &[String]| if builds.is_empty() {
        " <<= requires a build artifact"
    } else if validate_unique_values(builds) {
        ""
    } else {
        " <<= build artifacts need to be unique"
    };

    let check_associated = |associated: &[String]| if validate_unique_values(associated) {
        ""
    } else {
        " <<= associated files & folders need to be unique"
    };

    for platform in platforms {
        if separator { println!(); }

        skip_first.call_once(|| separator = true);

        display_platform(platform, check_name, check_builds, check_associated);
    }
}

/// Shows the path of the configuration json file
pub fn show_configuration() {
    println!("{}", path_of_supported_platforms_configuration().to_string_lossy());
}

/// Lets user interactively define and add a new platform to configuration
fn add_new_platform(platforms: &mut Vec<Platform>) -> bool {
    let Some(name) = get_platform_name(platforms, "") else { return false; };

    let folders = get_a_collection_of_input("Add build artifact", &[], true, true);

    if folders.is_empty() {
        eprintln!("\nYou must provide at least one build artifact\n");
        return false;
    }

    let associated = get_a_collection_of_input(
        "Add file or folder name that identifies platform", &[], false, true,
    )
        .into_iter()
        .map(Filter::new)
        .collect();

    let platform = Platform { name, folders, associated };

    if let Some(equivalent) = platforms.iter().find(|p| p.same_as(&platform)) {
        println!("\n{:?} handles the same build artifacts\n", equivalent.name);
        display_platform(equivalent, no_check, no_check, no_check);
        println!();

        return false;
    }

    println!();
    display_platform(&platform, no_check, no_check, no_check);
    println!();

    platforms.push(platform);

    platforms.sort();

    true
}

/// Prompts user to select a platform configuration name
fn get_platform_name(platforms: &[Platform], initial_value: &str) -> Option<String> {
    Text::new("Platform name:")
        .with_validator(&validate_not_blank)
        .with_validator(&validate_no_spaces)
        .with_validator(&validate_unique(platforms, "platform"))
        .with_initial_value(initial_value)
        .prompt()
        .ok()
}

/// Confirms user action
fn confirmed(message: &str) -> bool {
    let confirmation = Confirm::new(&format!("{message}, are you sure"))
        .with_default(false)
        .with_placeholder("N")
        .prompt();

    confirmation.unwrap_or(false)
}

/// Lets user interactively delete selected platforms
fn delete_platforms(platforms: &mut Vec<Platform>) -> bool {
    let choices = platforms.iter().map(|p| p.name.clone()).collect::<Vec<_>>();

    let Some(selected) = make_selections("platform(s) to delete", &choices) else { return false; };

    let mut modified = false;

    for platform in selected {
        let platform = platforms.iter().enumerate().find_map(
            |(idx, p)| if p.name == platform { Some(idx) } else { None }
        );

        if let Some(idx) = platform {
            platforms.remove(idx);
            modified = true;
        }
    }

    modified
}

/// Displays a platform to the user
fn display_platform<'a, N, B, A>(
    platform: &'a Platform,
    name_check: N,
    build_check: B,
    associate_check: A,
)
    where N: Fn(&'a str) -> &'static str + 'a,
          B: Fn(&'a [String]) -> &'static str + 'a,
          A: Fn(&'a [String]) -> &'static str + 'a
{
    println!("Platform:          {}{}", platform.name, name_check(&platform.name));
    println!("  Build Artifacts: {}{}", list_output(&platform.folders), build_check(&platform.folders));
    println!("  Matched On:      {}{}", list_output(&platform.associated), associate_check(&platform.folders));
}

/// Lets user select entries in a collection and deletes them
fn delete_selected_entries<V>(values: &mut Vec<V>, what: &str)
    where V: AsRef<str>
{
    if let Some(deleted) = make_selections(&format!("{what} to delete"), values) {
        for item in deleted {
            if let Some(index) = values.iter().position(|f| item == f.as_ref()) {
                values.remove(index);
            }
        }
    }
}

/// Prompts and receives a collection of user input, supports input string validation
fn get_a_collection_of_input(
    prompt: &str,
    validators: &[StringValidator],
    required: bool,
    unique: bool,
) -> Vec<String> {
    const AT_LEAST_ONE: &str = "Requires at least one value";
    const BLANK_TO_END: &str = "Leave blank to end";

    let mut collection = <HashMap<String, String>>::new();
    let mut message = if required { AT_LEAST_ONE } else { BLANK_TO_END };
    let end_message = Once::new();

    loop {
        let input = Text::new(prompt)
            .with_help_message(message)
            .with_validators(validators);

        let value = if unique {
            let new_values = collection.values().cloned().collect::<Vec<_>>();
            let unique_validator = &validate_unique(&new_values, "");

            input.with_validator(unique_validator).prompt()
        } else {
            input.prompt()
        };

        match value {
            Ok(input) if input.trim().is_empty() => break,
            Ok(input) => {
                let input = input.trim();

                collection.entry(input.to_lowercase()).or_insert_with(|| input.to_string());
            }
            Err(_) => break
        }

        end_message.call_once(|| message = AT_LEAST_ONE);
    }

    collection.into_values().collect()
}

/// Lets user interactively modify an existing platform
fn modify_a_platform(platforms: &mut [Platform]) -> bool {
    const RENAME: &str = "Rename";
    const ADD_ARTIFACT: &str = "Add Artifact";
    const REMOVE_ARTIFACTS: &str = "Remove Artifacts";
    const ADD_ASSOCIATED: &str = "Add Associated";
    const REMOVE_ASSOCIATED: &str = "Remove Associated";
    const ACCEPT: &str = "Accept Modifications";
    const CANCEL: &str = "Cancel Modifications";

    let choices = platforms.iter().map(|p| p.name.clone()).collect::<Vec<_>>();

    let Some(selected) = make_a_selection("a platform to modify:", &choices) else { return false; };

    let platform_index = platforms.iter().position(|p| p.name == selected).unwrap();
    let original_platform = &platforms[platform_index];
    let mut modified_platform = original_platform.clone();
    let mut modified = false;

    let choices = vec![
        RENAME,
        ADD_ARTIFACT, REMOVE_ARTIFACTS,
        ADD_ASSOCIATED, REMOVE_ASSOCIATED,
        ACCEPT, CANCEL,
    ];
    let modified_choices = vec![
        ACCEPT, CANCEL,
        RENAME,
        ADD_ARTIFACT, REMOVE_ARTIFACTS,
        ADD_ASSOCIATED, REMOVE_ASSOCIATED,
    ];

    loop {
        let choices = if modified { &modified_choices } else { &choices };

        let Some(choice) = make_a_selection("a modification:", choices) else { return false; };

        match choice.as_str() {
            RENAME =>
                if let Some(name) = get_platform_name(platforms, &modified_platform.name) {
                    modified_platform.name = name;
                },
            ADD_ARTIFACT => {
                let artifacts = get_a_collection_of_input(
                    "Add new artifacts",
                    &[&validate_unique(&modified_platform.folders, "artifacts")],
                    false, true,
                );

                for artifact in artifacts {
                    modified_platform.folders.push(artifact);
                }
            }
            REMOVE_ARTIFACTS =>
                delete_selected_entries(&mut modified_platform.folders, "artifacts"),
            ADD_ASSOCIATED => {
                let associated = get_a_collection_of_input(
                    "Add new associated",
                    &[&validate_unique(&modified_platform.associated, "associated")],
                    false, true,
                );

                for item in associated {
                    modified_platform.associated.push(Filter::new(item));
                }
            }
            REMOVE_ASSOCIATED =>
                delete_selected_entries(&mut modified_platform.associated, "associated"),
            ACCEPT =>
                if validate_platform(&modified_platform) { break; },
            CANCEL =>
                return false,
            _ => unreachable!()
        }

        modified = modified_platform != *original_platform || !modified_platform.same_as(original_platform);
    }

    let original_platform = &mut platforms[platform_index];

    if modified {
        *original_platform = modified_platform;

        println!();
        display_platform(original_platform, no_check, no_check, no_check);
        println!();
    }

    modified
}

/// Let a user interactively select a value from a collection of choices
fn make_a_selection<S>(message: &str, choices: &[S]) -> Option<String>
    where S: AsRef<str>
{
    let choices = choices.iter().map(|s| s.as_ref().to_string()).collect();

    Select::new(&format!("Select {message}"), choices).prompt().ok()
}

/// Let a user interactively selects values from a collection of choices
fn make_selections<S>(message: &str, choices: &[S]) -> Option<Vec<String>>
    where S: AsRef<str>
{
    let choices = choices.iter().map(|s| s.as_ref().to_string()).collect();
    let selected = MultiSelect::new(&format!("Select one or more {message}:"), choices).prompt();

    match selected {
        Ok(selected) if selected.is_empty() => None,
        Ok(selected) => Some(selected),
        Err(_) => None
    }
}

// constant functions cannot evaluate destructors
#[allow(clippy::missing_const_for_fn)]
fn no_check<F>(_: F) -> &'static str { "" }

/// Deletes supported platforms configuration file
fn reset_configuration_json() {
    let path = path_of_supported_platforms_configuration();

    if path.exists() {
        match remove_file(&path) {
            Ok(()) => println!("Configuration of supported platforms has been reset"),
            Err(err) => display_error_and_exit(&format!("Exception resenting configuration: {err}"))
        }
    }
}

/// Saves platform configuration
fn save_platforms(platforms: &[Platform]) -> bool {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path_of_supported_platforms_configuration());

    let Ok(file) = file else { return false; };

    let writer = BufWriter::new(file);

    if let Err(err) = serde_json::to_writer(writer, platforms) {
        println!("\nException saving configuration: {err}\n");
        false
    } else {
        true
    }
}

/// Validates a value does not contain spaces
fn validate_no_spaces(value: &str) -> Result<(), String> {
    if value.contains(' ') {
        Err(String::from("Can not contain spaces"))
    } else {
        Ok(())
    }
}

/// Validates a value is no empty or blanks only
fn validate_not_blank(value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(String::from("Can not be blank"))
    } else {
        Ok(())
    }
}

/// Validates a value is unique in a collection of values
fn validate_unique<'a, V>(checked: &'a [V], what: &'a str) -> impl Fn(&str) -> Result<(), String> + 'a
    where V: AsRef<str>
{
    move |value: &str| if checked.iter().any(|v| v.as_ref().eq_ignore_ascii_case(value)) {
        Err(String::from(&format!("{value:?} already exists, {what} values must be unique")))
    } else {
        Ok(())
    }
}
