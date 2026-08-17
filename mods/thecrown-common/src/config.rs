use anyhow::Context;
use serde::de::DeserializeOwned;
use std::path::Path;

pub fn parse_toml_config<T: DeserializeOwned>(path: impl AsRef<Path>) -> anyhow::Result<T> {
    let path = path.as_ref();
    let source = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config {}", path.display()))?;
    toml::from_str(&source)
        .with_context(|| format!("failed to parse config {}", path.display()))
}
