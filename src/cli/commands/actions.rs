/// Defines common actions for commands
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum CommonAction {
    /// List matching folders, default subcommand
    List,

    /// Remove matching folders
    Remove,
}

/// Defines actions for supported platforms
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum SupportedAction {
    /// List configured development platforms
    List,

    /// Show path of platform configuration file
    Path,

    /// Manage platform configuration
    Manage,

    /// Reset platform configuration to default
    Reset,
}
