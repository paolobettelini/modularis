use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CrackedDeepslateTilesBlock;

impl Block for CrackedDeepslateTilesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cracked-deepslate-tiles",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CrackedDeepslateTilesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cracked-deepslate-tiles:block/cracked_deepslate_tiles"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CrackedDeepslateTilesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CrackedDeepslateTilesBlock::RENDER;

pub struct BlockCrackedDeepslateTilesMod;

impl BlockCrackedDeepslateTilesMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
