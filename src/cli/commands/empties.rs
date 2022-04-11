use crate::cli::commands::actions::Action;

/// Empties subcommand for handling empty folders
#[derive(Debug, Eq, PartialEq, Args)]
pub struct Empties {
    /// Optionally specify action, defaults to "list"
    #[clap(subcommand)]
    pub action: Option<Action>,

    /// Executes remove action non-interactively,, defaults to interactive
    #[clap(short = 'y', long, verbatim_doc_comment)]
    pub confirmed: bool,

    /// Optionally specify target path, defaults to current folder
    #[clap(short, long, verbatim_doc_comment, default_value = ".")]
    pub path: String,

    /// Includes empty hidden folders, i.e. folders that start with a '.'
    #[clap(short = 's', long, verbatim_doc_comment)]
    pub hidden: bool,
}