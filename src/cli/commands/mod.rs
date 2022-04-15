use crate::cli::commands::builds::Builds;
use crate::cli::commands::empties::Empties;
use crate::cli::commands::supported::Supported;

pub mod builds;
pub mod empties;
pub mod supported;

/// `cleaner` subcommands defined
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum Commands {
    /// Manage build artifacts of supported platforms
    #[clap(subcommand)]
    Builds(Builds),

    /// Manage empty folders
    #[clap(subcommand)]
    Empties(Empties),

    /// Manage supported development platforms
    #[clap(subcommand)]
    Supported(Supported),
}
