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

    pub const fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Infinite, _) | (_, Self::Infinite) => Self::Infinite,
            (Self::Finite(left), Self::Finite(right)) => Self::Finite(left.saturating_add(right)),
        }
    }

    /// Returns the metadata value after one successful use.
    ///
    /// `None` means that the item is depleted and its cell should become
    /// empty. Infinite quantities remain infinite.
    pub const fn after_one_use(self) -> Option<Self> {
        match self {
            Self::Infinite => Some(Self::Infinite),
            Self::Finite(0 | 1) => None,
            Self::Finite(value) => Some(Self::Finite(value - 1)),
        }
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
