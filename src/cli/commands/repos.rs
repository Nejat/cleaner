/// Repos subcommand
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum Repos {
    /// List repositories not in master or main
    Branched {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List repositories with uncommitted changes
    Changes {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List detached repositories, HEAD
    Detached {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List repositories with errors
    Error {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List repositories that are only initialized (unborn)
    Init {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List repositories with no remotes configured
    Local {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List repositories with a "Main" branch
    Main {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List repositories with a "Master" branch
    Master {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List outdated repos
    Outdated {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,

        /// Filters outdated repos 
        #[clap(required = false, short = 'f', long, verbatim_doc_comment, default_value = "either")]
        filter: OutdatedFilter,

        /// Only check repo's Main branch
        #[clap(short = 'm', long, verbatim_doc_comment)]
        main: bool,
    },
    /// List repositories that are up-to-date
    UpToDate {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,

        /// Only check repo's Main branch
        #[clap(short = 'm', long, verbatim_doc_comment)]
        main: bool,
    },
}

/// Outdated Filter
#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
#[clap(verbatim_doc_comment)]
pub enum OutdatedFilter {
    /// Only include outdated repos that are ahead in commits of the remote  
    #[clap(verbatim_doc_comment)]
    Ahead,
    /// Either Ahead or Behind (Default Value)
    #[clap(verbatim_doc_comment)]
    Either,
    /// Only include outdated repos that are behind in commits of the remote
    #[clap(verbatim_doc_comment)]
    Behind,
}