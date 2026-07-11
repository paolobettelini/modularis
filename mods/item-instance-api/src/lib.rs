pub use generated_item_metadata::ItemMetaSet;
pub use generated_item_registry::{ItemId, all_items};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemInstance {
    pub item: ItemId,
    pub metadata: ItemMetaSet,
}

impl ItemInstance {
    pub fn new(item: ItemId) -> Self {
        Self {
            item,
            metadata: ItemMetaSet::default(),
        }
    }

    pub fn with_metadata(item: ItemId, metadata: ItemMetaSet) -> Self {
        Self { item, metadata }
    }
}
