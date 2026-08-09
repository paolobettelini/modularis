use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeepslateTilesBlock;

impl Block for DeepslateTilesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:deepslate-tiles",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeepslateTilesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-deepslate-tiles:block/deepslate_tiles"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeepslateTilesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeepslateTilesBlock::RENDER;

pub struct BlockDeepslateTilesMod;

impl BlockDeepslateTilesMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
