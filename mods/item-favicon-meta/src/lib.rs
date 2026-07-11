use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemFavicon {
    pub path: String,
}

impl ItemFavicon {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

pub struct ItemFaviconMetaMod;

impl ItemFaviconMetaMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
