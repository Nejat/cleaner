use crate::cli::commands::actions::SupportedAction;

/// Builds subcommand for handling build artifacts for supported platforms
#[derive(Debug, Eq, PartialEq, Args)]
pub struct Supported {
    /// Optionally specify action, defaults to "list"
    #[clap(subcommand)]
    pub action: Option<SupportedAction>,
}