/// Empties subcommand for handling empty folders
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum Empties {
    /// List matching empty folders
    #[clap(alias = "ls")]
    List {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,

        /// Includes empty hidden folders, i.e. folders that start with a '.'
        #[clap(short = 's', long, verbatim_doc_comment)]
        hidden: bool,
    },
    /// Remove matching empty folders
    #[clap(alias = "rm")]
    Remove {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,

        /// Executes remove action without confirmation, defaults to interactive confirmation
        #[clap(short = 'y', long, verbatim_doc_comment)]
        confirmed: bool,

        /// Includes empty hidden folders, i.e. folders that start with a '.'
        #[clap(short = 's', long, verbatim_doc_comment)]
        hidden: bool,
    },
}