use std::{borrow::Borrow, fmt, ops::Deref};

use serde::{Deserialize, Deserializer, *};

use crate::static_str;

/// A thin wrapper around `&'static str` that can be (de)serialized
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct StaticStr(&'static str);

impl StaticStr {
    /// Create a new `StaticStr`
    pub fn new(s: &str) -> Self {
        StaticStr(static_str(s))
    }
}

impl fmt::Debug for StaticStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for StaticStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for StaticStr {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl AsRef<str> for StaticStr {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl Borrow<str> for StaticStr {
    fn borrow(&self) -> &str {
        self.0
    }
}

impl<'de> Deserialize<'de> for StaticStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <String as Deserialize<'de>>::deserialize(deserializer).map(|s| StaticStr::new(&s))
    }
}

impl<'a> From<&'a str> for StaticStr {
    fn from(s: &'a str) -> Self {
        StaticStr::new(s)
    }
}

impl From<String> for StaticStr {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}
