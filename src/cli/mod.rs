#![allow(clippy::upper_case_acronyms)]

use crate::cli::commands::Commands;

pub mod commands;
pub mod all_values;

/// Utility for cleaning build artifacts in bulk
///
/// Manage local build artifact folders for common dev platforms,
/// including: Rust, .Net & Web
#[derive(Debug, Parser)]
#[clap(version, about, long_about = None, verbatim_doc_comment)]
#[clap(propagate_version = true)]
pub struct CLI {
    /// Defines cli commands
    #[clap(subcommand)]
    pub commands: Commands,
}