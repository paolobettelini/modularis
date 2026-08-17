use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShowExternalLink {
    pub title: String,
    pub description: String,
    pub url: String,
}
