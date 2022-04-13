use crate::cli::commands::builds::Builds;
use crate::cli::commands::empties::Empties;
use crate::cli::commands::supported::Supported;

pub mod actions;
pub mod builds;
pub mod empties;
pub mod supported;

/// `cleaner` subcommands defined
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum Commands {
    /// Manage build artifacts of supported platforms
    Builds(Builds),

    /// Manage empty folders
    Empties(Empties),

    /// Manage supported development platforms
    Supported(Supported),
}
