/// Supported subcommand for managing supported platforms
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum Supported {
    /// List configured development platforms
    #[clap(alias = "ls")]
    List,

    /// Show path of platform configuration file
    #[clap(alias = "show")]
    Path,

    /// Manage platform configuration
    Manage,

    /// Reset platform configuration to default
    Reset,
}