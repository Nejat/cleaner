/// Supported subcommand for managing supported platforms
#[derive(Debug, Eq, PartialEq, Subcommand)]
#[clap(verbatim_doc_comment)]
pub enum Supported {
    /// List configured development platforms
    #[clap(alias = "ls", verbatim_doc_comment)]
    List,

    /// Show path of platform configuration file
    #[clap(alias = "show", verbatim_doc_comment)]
    Path,

    /// Manage platform configuration
    Manage,

    /// Reset platform configuration to default
    Reset {
        /// Executes reset without confirmation, defaults to interactive confirmation
        #[clap(short = 'y', long, verbatim_doc_comment)]
        confirmed: bool,
    },
}