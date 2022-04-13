use crate::cli::all_values::AllValues;
use crate::cli::commands::actions::CommonAction;

/// Builds subcommand for handling build artifacts for supported platforms
#[derive(Debug, Eq, PartialEq, Args)]
pub struct Builds {
    /// Optionally specify action, defaults to "list"
    #[clap(subcommand)]
    pub action: Option<CommonAction>,

    /// Executes remove action non-interactively,, defaults to interactive
    #[clap(short = 'y', long, verbatim_doc_comment)]
    pub confirmed: bool,

    /// Optionally specify target path, defaults to current folder
    #[clap(short, long, verbatim_doc_comment, default_value = ".")]
    pub path: String,

    /// Optionally specify supported development platform(s), defaults to "all"
    ///
    /// * use "supported" command to see a list of all supported
    /// development platforms
    #[clap(short, long, verbatim_doc_comment, default_value_t = AllValues::All)]
    pub types: AllValues,
}