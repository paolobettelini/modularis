use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct WorldInstanceId(pub String);

impl WorldInstanceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorldInstanceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorldScopeId {
    pub instance: WorldInstanceId,
    pub source: String,
}

impl WorldScopeId {
    pub fn new(instance: WorldInstanceId, source: impl Into<String>) -> Self {
        Self {
            instance,
            source: source.into(),
        }
    }
}
