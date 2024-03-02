use crate::cli::commands::builds::Builds;
use crate::cli::commands::empties::Empties;
use crate::cli::commands::repos::Repos;
use crate::cli::commands::supported::Supported;

pub mod builds;
pub mod empties;
pub mod repos;
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

    /// Search through repos
    #[clap(subcommand)]
    Repos(Repos),

    /// Manage supported development platforms
    #[clap(subcommand)]
    Supported(Supported),
}
