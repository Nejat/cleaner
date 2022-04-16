#![doc = include_str ! ("../README.md")]

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
// ==============================================================
#![doc(html_root_url = "https://docs.rs/cleaner/0.9.1")]

#[macro_use]
extern crate clap;

use clap::Parser;
use once_cell::sync::Lazy;

use commands::builds::list_build_artifacts;
use commands::builds::remove_build_artifacts;

use crate::cli::all_values::AllValues;
use crate::cli::CLI;
use crate::cli::commands::builds::Builds;
use crate::cli::commands::Commands;
use crate::cli::commands::empties::Empties;
use crate::cli::commands::supported::Supported;
use crate::commands::empties::{list_empties, remove_empties};
use crate::commands::supported::{manage_configuration, reset_configuration, show_configuration, supported_platforms};
use crate::models::Platform;
use crate::utils::load_supported_platforms;

#[doc(hidden)]
mod cli;
#[doc(hidden)]
mod commands;
#[doc(hidden)]
mod models;
#[doc(hidden)]
mod utils;

#[cfg(test)]
mod tests;

/// Definition of supported development platforms
#[doc(hidden)]
static PLATFORMS: Lazy<Vec<Platform>> = Lazy::new(load_supported_platforms);

/// Cleaner command line parsing and command execution
#[doc(hidden)]
fn main() {
    let cli = CLI::parse();

    println!();

    match &cli.commands {
        Commands::Builds(Builds::List { path, types }) =>
            list_build_artifacts(path, types, &PLATFORMS),
        Commands::Builds(Builds::Remove { path, types, confirmed }) =>
            remove_build_artifacts(path, types, &PLATFORMS, *confirmed),
        Commands::Empties(Empties::List { path, hidden }) =>
            list_empties(path, *hidden),
        Commands::Empties(Empties::Remove { path, confirmed, hidden }) =>
            remove_empties(path, *confirmed, *hidden),
        Commands::Supported(Supported::List) =>
            supported_platforms(&PLATFORMS),
        Commands::Supported(Supported::Path) =>
            show_configuration(),
        Commands::Supported(Supported::Manage) =>
            manage_configuration(),
        Commands::Supported(Supported::Reset { confirmed }) =>
            reset_configuration(*confirmed),
    }

    println!();
}
