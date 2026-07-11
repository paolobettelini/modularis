use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortalIgniter;

impl PortalIgniter {
    pub const fn new() -> Self {
        Self
    }
}

impl Default for PortalIgniter {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ItemPortalIgniterMetaMod;

impl ItemPortalIgniterMetaMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
