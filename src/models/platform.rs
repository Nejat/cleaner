use std::cmp::Ordering;

use crate::models::filter::Filter;

/// Describes a supported development platform
#[derive(Clone, Serialize, Deserialize)]
pub struct Platform {
    /// Development platform name
    pub name: String,

    /// Expected build artifact folder names of platform
    pub folders: Vec<String>,

    /// Associated files and file extensions that mark the platform
    pub associated: Vec<Filter>,
}

impl Platform {
    /// Checks if two supported platforms produce the same effect
    pub fn same_as(&self, other: &Self) -> bool {
        self.folders.len() == other.folders.len() &&
            self.associated.len() == other.associated.len() &&
            self.folders.iter()
                .all(|f| other.folders.iter().any(|f2| f.trim().eq_ignore_ascii_case(f2.trim()))) &&
            self.associated.iter()
                .all(|f| other.associated.iter().any(|f2| f.as_ref().trim().eq_ignore_ascii_case(f2.as_ref().trim())))
    }
}

impl AsRef<str> for Platform {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl Eq for Platform {}

impl PartialEq<Self> for Platform {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
    }
}

impl Ord for Platform {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Platform {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
