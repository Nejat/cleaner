#![doc = include_str ! ("../README.md")]

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
// ==============================================================
#![doc(html_root_url = "https://docs.rs/cleaner/0.2.0")]

#[macro_use]
extern crate clap;

use clap::Parser;
use once_cell::sync::Lazy;

use cli::commands::actions::Action;
use commands::builds::list_build_artifacts;
use commands::builds::remove_build_artifacts;

use crate::cli::all_values::AllValues;
use crate::cli::CLI;
use crate::cli::commands::Commands;
use crate::commands::supported::supported_platforms;
use crate::models::Platform;

//#[doc(hidden)]
mod cli;
//#[doc(hidden)]
mod commands;
//#[doc(hidden)]
mod models;
//#[doc(hidden)]
mod utils;

#[cfg(test)]
mod tests;

/// Definition of supported development platforms
//#[doc(hidden)]
static PLATFORMS: Lazy<Vec<Platform>> = Lazy::new(|| {
    // todo: make this configurable by loading from a json definition file
    vec![
        Platform {
            name: ".Net",
            folders: vec!["bin", "obj"],
            associated: vec!["sln", "csproj"],
        },
        Platform {
            name: "Rust",
            folders: vec!["target"],
            associated: vec!["cargo.toml"],
        },
        Platform {
            name: "Web",
            folders: vec!["node_modules"],
            associated: vec!["package.json"],
        },
    ]
});

/// Cleaner command line parsing and command execution
//#[doc(hidden)]
fn main() {
    let cli = CLI::parse();

    println!();

    match &cli.commands {
        Commands::Builds(builds) => {
            match builds.action {
                None |
                Some(Action::List) =>
                    list_build_artifacts(&builds.path, &builds.types, &PLATFORMS),
                Some(Action::Remove) =>
                    remove_build_artifacts(&builds.path, &builds.types, &PLATFORMS, builds.confirmed)
            }
        }
        Commands::Supported => supported_platforms(&PLATFORMS)
    }

    println!();
}
