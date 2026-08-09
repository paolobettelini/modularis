use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeepslateCoalOreBlock;

impl Block for DeepslateCoalOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:deepslate-coal-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeepslateCoalOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-deepslate-coal-ore:block/deepslate_coal_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeepslateCoalOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeepslateCoalOreBlock::RENDER;

pub struct BlockDeepslateCoalOreMod;

impl BlockDeepslateCoalOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
