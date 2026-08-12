use block_component_api::BlockComponent;
use block_properties_api::BlockProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockDurability {
    Breakable(u32),
    Unbreakable,
}

impl BlockDurability {
    pub const fn breakable(points: u32) -> Self {
        Self::Breakable(points)
    }

    pub const fn points(self) -> Option<u32> {
        match self {
            Self::Breakable(points) => Some(points),
            Self::Unbreakable => None,
        }
    }
}

pub const COMMON_BLOCK_DURABILITY: BlockDurability = BlockDurability::breakable(100);

pub struct BlockDurabilityProperty;
impl BlockProperty for BlockDurabilityProperty {
    type Value = BlockDurability;
    const ID: &'static str = "vanilla:durability";
    fn default_value() -> Self::Value { COMMON_BLOCK_DURABILITY }
}

/// Sparse per-position delta. Zero is represented by absence from the store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockDamage(pub u32);

impl BlockComponent for BlockDamage {
    const ID: &'static str = "vanilla:block-damage";
    const VERSION: u32 = 1;
}
