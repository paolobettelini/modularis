use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Quantity {
    Finite(u32),
    Infinite,
}

impl Quantity {
    pub const fn is_empty(self) -> bool {
        matches!(self, Self::Finite(0))
    }
}

pub struct ItemQuantityMetaMod;

impl ItemQuantityMetaMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
