use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

/// Logical grass plant. Its authored model carries only the selection shape;
/// visible blades are supplied by an optional client renderer.
pub struct ShortGrassBlock;

impl Block for ShortGrassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:short_grass",
        is_air: false,
        solid: false,
        opaque: false,
    };
}

impl BlockRender for ShortGrassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-short-grass:block/short_grass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ShortGrassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ShortGrassBlock::RENDER;

pub struct BlockShortGrassMod;

impl BlockShortGrassMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
