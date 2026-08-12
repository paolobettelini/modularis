pub use generated_block_state::BlockStateSet;
pub use generated_block_registry::{
    BlockId, all_blocks, from_str as block_id_from_str, id as block_id_as_str,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockState {
    pub block: BlockId,
    pub state: BlockStateSet,
}

impl BlockState {
    pub fn new(block: BlockId) -> Self {
        Self {
            block,
            state: BlockStateSet::default(),
        }
    }

    pub fn with_state(block: BlockId, state: BlockStateSet) -> Self {
        Self { block, state }
    }
}

impl From<BlockId> for BlockState {
    fn from(block: BlockId) -> Self {
        Self::new(block)
    }
}
