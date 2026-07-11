use block_api::BlockInfo;
use block_render_api::BlockRenderInfo;
pub use generated_block_registry::BlockId;

pub trait BlockManagerApi: Send + Sync + 'static {
    fn info(block: BlockId) -> &'static BlockInfo;
    fn render_info(block: BlockId) -> &'static BlockRenderInfo;
    fn all() -> &'static [BlockId];
    fn from_string(id: &str) -> Option<BlockId>;
    fn id(block: BlockId) -> &'static str;

    fn is_air(block: BlockId) -> bool {
        Self::info(block).is_air
    }

    fn is_solid(block: BlockId) -> bool {
        Self::info(block).solid
    }

    fn is_opaque(block: BlockId) -> bool {
        Self::info(block).opaque
    }
}
