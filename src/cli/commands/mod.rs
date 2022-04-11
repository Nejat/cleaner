use crate::cli::commands::builds::Builds;

pub mod actions;
pub mod builds;

/// `cleaner` subcommands defined
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum Commands {
    /// Manage build artifacts of supported platforms
    Builds(Builds),

    /// List supported development platforms
    Supported,
}
