use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ReinforcedDeepslateBlock;

impl Block for ReinforcedDeepslateBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:reinforced-deepslate",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ReinforcedDeepslateBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-reinforced-deepslate:block/reinforced_deepslate"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ReinforcedDeepslateBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ReinforcedDeepslateBlock::RENDER;

pub struct BlockReinforcedDeepslateMod;

impl BlockReinforcedDeepslateMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
