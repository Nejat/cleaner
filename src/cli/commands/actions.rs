/// Defines actions for build artifacts
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum Action {
    /// List matching folders, default subcommand
    List,

    /// Remove matching folders
    Remove,
}
