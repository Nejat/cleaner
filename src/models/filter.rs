use std::fmt::Formatter;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use wildmatch::WildMatch;

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

impl<'de> Deserialize<'de> for Filter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let value = deserializer.deserialize_str(StringVisitor)?;

        Ok(Self::new(value))
    }
}

struct StringVisitor;

impl Visitor<'_> for StringVisitor {
    type Value = String;

    fn expecting(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("a string representing a file name, wildcards [*, ?] accepted")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: Error {
        Ok(value.to_string())
    }
}
