use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeepslateGoldOreBlock;

impl Block for DeepslateGoldOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:deepslate-gold-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeepslateGoldOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-deepslate-gold-ore:block/deepslate_gold_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeepslateGoldOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeepslateGoldOreBlock::RENDER;

pub struct BlockDeepslateGoldOreMod;

impl BlockDeepslateGoldOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
