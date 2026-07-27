pub use generated_block_metadata::BlockMetaSet;
pub use generated_block_registry::{
    BlockId, all_blocks, from_str as block_id_from_str, id as block_id_as_str,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockInstance {
    pub block: BlockId,
    pub metadata: BlockMetaSet,
}

impl BlockInstance {
    pub fn new(block: BlockId) -> Self {
        Self {
            block,
            metadata: BlockMetaSet::default(),
        }
    }

    pub fn with_metadata(block: BlockId, metadata: BlockMetaSet) -> Self {
        Self { block, metadata }
    }
}

impl From<BlockId> for BlockInstance {
    fn from(block: BlockId) -> Self {
        Self::new(block)
    }
}
