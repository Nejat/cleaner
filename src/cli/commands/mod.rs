use crate::cli::commands::builds::Builds;
use crate::cli::commands::empties::Empties;

pub mod actions;
pub mod builds;
pub mod empties;

/// `cleaner` subcommands defined
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum Commands {
    /// Manage build artifacts of supported platforms
    Builds(Builds),

    /// Manage empty folders
    Empties(Empties),

    /// List supported development platforms
    Supported,
}
