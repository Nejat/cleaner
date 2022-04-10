/// Defines actions for build artifacts
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum Action {
    /// List corresponding folders, default subcommand
    List,

    /// Remove corresponding folders
    Remove,
}
