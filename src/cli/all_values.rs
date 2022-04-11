use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::utils::SEPARATOR;

/// Generic comma delimited multiple string values or "all" argument
#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum AllValues {
    /// Indicates "all" should be included
    #[clap(verbatim_doc_comment)]
    All,

    /// Defines only specific values
    #[clap(verbatim_doc_comment)]
    Values {
        values: Vec<String>
    },
}

impl AllValues {
    /// Helper method for selecting between two value based on variant
    #[allow(dead_code)] // i likes me some symmetries
    pub const fn for_all<'a, T: ?Sized>(&'a self, value: &'a T, other: &'a T) -> &'a T {
        match self {
            Self::All => value,
            Self::Values { .. } => other,
        }
    }

    /// Helper method for selecting between two value based on variant
    pub const fn for_select<'a, T: ?Sized>(&'a self, value: &'a T, other: &'a T) -> &'a T {
        match self {
            Self::All => other,
            Self::Values { .. } => value,
        }
    }

    /// Checks to see if value is included
    pub fn matches(&self, checked: &str) -> bool {
        match self {
            Self::All => true,
            Self::Values { values } => values.iter().any(|v| v == checked)
        }
    }

    /// Helper method for determining if input requires pluralization
    pub fn pluralize<'a>(&'a self, plural: &'a str) -> &'a str {
        match self {
            Self::All => plural,
            Self::Values { values } if values.len() > 1 => plural,
            Self::Values { .. } => "",
        }
    }
}

impl Display for AllValues {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All =>
                fmt.write_str("all"),
            Self::Values { values } =>
                fmt.write_str(&values.join(SEPARATOR))
        }
    }
}

impl FromStr for AllValues {
    type Err = String;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Ok(if src.trim().to_lowercase() == "all" {
            Self::All
        } else {
            Self::Values {
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
