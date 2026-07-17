use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};

use crate::{Error, Result};

/// A namespaced Minecraft identifier such as `minecraft:block/stone`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceLocation {
    namespace: String,
    path: String,
}

impl ResourceLocation {
    pub fn new(namespace: impl Into<String>, path: impl Into<String>) -> Result<Self> {
        let namespace = namespace.into();
        let path = path.into();
        validate_namespace(&namespace)?;
        validate_path(&path)?;
        Ok(Self { namespace, path })
    }

    pub fn parse(value: &str) -> Result<Self> {
        let (namespace, path) = value
            .split_once(':')
            .map(|(namespace, path)| (namespace, path))
            .unwrap_or(("minecraft", value));
        Self::new(namespace, path)
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn with_path(&self, path: impl Into<String>) -> Result<Self> {
        Self::new(self.namespace.clone(), path)
    }
}

fn validate_namespace(value: &str) -> Result<()> {
    if value.is_empty()
        || !value.bytes().all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'_' | b'-' | b'.')
        })
    {
        return Err(Error::InvalidResourceLocation(value.to_owned()));
    }
    Ok(())
}

fn validate_path(value: &str) -> Result<()> {
    if value.is_empty()
        || value.starts_with('/')
        || value.ends_with('/')
        || value
            .split('/')
            .any(|segment| segment == ".." || segment.is_empty())
        || !value.bytes().all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'_' | b'-' | b'.' | b'/')
        })
    {
        return Err(Error::InvalidResourceLocation(value.to_owned()));
    }
    Ok(())
}

impl fmt::Display for ResourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl FromStr for ResourceLocation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::parse(s)
    }
}

impl Serialize for ResourceLocation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ResourceLocation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}
