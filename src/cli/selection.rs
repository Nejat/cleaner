use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::utils::list_output;

/// Generic comma delimited multiple string values or "all" argument
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum Selection {
    /// Indicates "all" should be included
    #[clap(verbatim_doc_comment)]
    All,

    /// Defines only specific values
    #[clap(verbatim_doc_comment)]
    Select {
        values: Vec<String>
    },
}

impl Selection {
    /// Helper method for selecting between two values based on variant value
    pub const fn choose<'a, T: ?Sized>(&'a self, select: &'a T, all: &'a T) -> &'a T {
        match self {
            Self::All => all,
            Self::Select { .. } => select,
        }
    }

    /// Checks to see if value is included
    pub fn matches(&self, checked: &str) -> bool {
        match self {
            Self::All => true,
            Self::Select { values } => values.iter().any(|v| v.eq_ignore_ascii_case(checked))
        }
    }

    /// Helper method for determining if input requires pluralization
    pub fn pluralize<'a>(&'a self, plural: &'a str) -> &'a str {
        match self {
            Self::All => plural,
            Self::Select { values } if values.len() > 1 => plural,
            Self::Select { .. } => "",
        }
    }
}

impl Display for Selection {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All =>
                fmt.write_str("all"),
            Self::Select { values } =>
                fmt.write_str(&list_output(values))
        }
    }
}

impl FromStr for Selection {
    type Err = String;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Ok(if src.trim().to_lowercase() == "all" {
            Self::All
        } else {
            Self::Select {
                values: src.split(',').filter_map(
                    |v| if v.trim().is_empty() {
                        None
                    } else {
                        Some(v.trim().to_string())
                    }
                ).collect()
            }
        })
    }
}
