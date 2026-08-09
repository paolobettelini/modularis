use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GoldOreBlock;

impl Block for GoldOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:gold-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GoldOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-gold-ore:block/gold_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GoldOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GoldOreBlock::RENDER;

pub struct BlockGoldOreMod;

impl BlockGoldOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
