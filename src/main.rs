#![doc = include_str!("../README.md")]

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::multiple_crate_versions)] // multiple bitflags versions
// ==============================================================
#![doc(html_root_url = "https://docs.rs/cleaner/0.10.1")]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde;

use clap::Parser;
use once_cell::sync::Lazy;

use commands::builds::list_build_artifacts;
use commands::builds::remove_build_artifacts;

use crate::cli::CLI;
use crate::cli::commands::builds::Builds;
use crate::cli::commands::Commands;
use crate::cli::commands::empties::Empties;
use crate::cli::commands::repos::Repos;
use crate::cli::commands::supported::Supported;
use crate::cli::selection::Selection;
use crate::commands::empties::{list_empties, remove_empties};
use crate::commands::repos::{
    list_outdated_repos, list_repos, list_repos_that_are_branched,
    list_repos_that_are_init_only, list_repos_with_branch,
    list_repos_with_detached_head, list_repos_with_errors,
    list_repos_with_uncommitted_changes,
    list_repos_without_configured_remotes, list_up_to_date_repos,
};
use crate::commands::supported::{
    manage_configuration, reset_configuration, show_configuration, supported_platforms,
};
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
        Commands::Repos(Repos::Branched { path }) =>
            list_repos_that_are_branched(path),
        Commands::Repos(Repos::Changes { path }) =>
            list_repos_with_uncommitted_changes(path),
        Commands::Repos(Repos::Detached { path }) =>
            list_repos_with_detached_head(path),
        Commands::Repos(Repos::Error { path }) =>
            list_repos_with_errors(path),
        Commands::Repos(Repos::Init { path }) =>
            list_repos_that_are_init_only(path),
        Commands::Repos(Repos::List { path }) =>
            list_repos(path),
        Commands::Repos(Repos::Local { path }) =>
            list_repos_without_configured_remotes(path),
        Commands::Repos(Repos::Main { path }) =>
            list_repos_with_branch(path, "main"),
        Commands::Repos(Repos::Master { path }) =>
            list_repos_with_branch(path, "master"),
        Commands::Repos(Repos::Outdated { path, filter, main }) =>
            list_outdated_repos(path, *filter, *main),
        Commands::Repos(Repos::UpToDate { path, main }) =>
            list_up_to_date_repos(path, *main)
    }

    println!();
}
