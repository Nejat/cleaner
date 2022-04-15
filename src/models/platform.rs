use std::cmp::Ordering;
use std::fmt::Formatter;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use wildmatch::WildMatch;

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
    /// Checks if two supported platforms produce the same affect
    pub fn same_as(&self, other: &Self) -> bool {
        self.folders.len() == other.folders.len() &&
        self.associated.len() == other.associated.len() &&
        self.folders.iter()
            .all(|f| other.folders.iter().any(|f2| f.trim().eq_ignore_ascii_case(f2.trim()))) &&
        self.associated.iter()
            .all(|f| other.associated.iter().any(|f2| f.1.trim().eq_ignore_ascii_case(f2.1.trim())))
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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

#[derive(Debug, Clone)]
pub struct Filter(WildMatch, String);

impl Filter {
    /// Initializes a new `Filter`
    pub fn new(filter: String) -> Self {
        Self(WildMatch::new(&filter), filter)
    }

    /// Validates a `Filter` matches a checked value
    pub fn matches(&self, value: impl AsRef<str>) -> bool {
        self.0.matches(value.as_ref())
    }
}

impl AsRef<str> for Filter {
    fn as_ref(&self) -> &str {
        &self.1
    }
}

impl Serialize for Filter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.as_ref())
    }
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("a string representing a file name, wildcards [*, ?] accepted")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: Error {
        Ok(value.to_string())
    }
}

impl<'de> Deserialize<'de> for Filter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let value = deserializer.deserialize_str(StringVisitor)?;

        Ok(Self::new(value))
    }
}
