use std::fmt::Formatter;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use wildmatch::WildMatch;

/// Describes a supported development platform
#[derive(Serialize, Deserialize)]
pub struct Platform {
    /// Development platform name
    pub name: String,

    /// Expected build artifact folder names of platform
    pub folders: Vec<String>,

    /// Associated files and file extensions that mark the platform
    pub associated: Vec<Filter>,
}

impl AsRef<str> for Platform {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

pub struct Filter(WildMatch, String);

impl Filter {
    pub fn matches(&self, value: &str) -> bool {
        self.0.matches(value)
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

        Ok(Self(WildMatch::new(&value), value))
    }
}
