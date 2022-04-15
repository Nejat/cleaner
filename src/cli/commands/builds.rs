use crate::cli::all_values::AllValues;

/// Builds subcommand for handling build artifacts for supported platforms
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum Builds {
    /// List matching build artifacts
    #[clap(alias = "ls")]
    List {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,

        /// Optionally specify supported development platform(s), defaults to "all"
        ///
        /// * use "supported" command to see a list of all supported development platforms
        #[clap(required = false, short, long, verbatim_doc_comment, default_value_t = AllValues::All)]
        types: AllValues,
    },
    /// Remove matching build artifacts
    #[clap(alias = "rm")]
    Remove {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,

        /// Optionally specify supported development platform(s), defaults to "all"
        ///
        /// * use "supported" command to see a list of all supported development platforms
        #[clap(short, long, verbatim_doc_comment, default_value_t = AllValues::All)]
        types: AllValues,

        /// Executes remove action without confirmation, defaults to interactive confirmation
        #[clap(short = 'y', long, verbatim_doc_comment)]
        confirmed: bool,
    }
}

impl Default for Builds {
    fn default() -> Self {
        Self::List {
            path: String::from('.'),
            types: AllValues::All
        }
    }
}