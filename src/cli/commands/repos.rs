/// Repos subcommand
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum Repos {
    /// List repositories not in master or main
    #[command(alias = "br")]
    Branched {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List repositories with uncommitted changes
    #[command(alias = "chg")]
    Changes {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List detached repositories, HEAD
    #[command(alias = "de")]
    Detached {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List repositories with errors
    #[command(alias = "err")]
    Error {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List repositories that are only initialized (unborn)
    #[command(alias = "unborn", alias = "ub", alias = "i")]
    Init {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List repositories
    #[command(alias = "ls")]
    List {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List repositories with no remotes configured
    #[command(alias = "lcl")]
    Local {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List repositories with a "Main" branch
    #[command(alias = "mn")]
    Main {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List repositories with a "Master" branch
    #[command(alias = "ms")]
    Master {
        /// Optionally specify target path, defaults to current folder
        #[clap(required = false, verbatim_doc_comment, default_value = ".")]
        path: String,
    },
    /// List outdated repos
    #[command(alias = "od")]
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
    #[command(alias = "utd", alias = "synced")]
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