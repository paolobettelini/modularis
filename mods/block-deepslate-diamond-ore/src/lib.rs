use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeepslateDiamondOreBlock;

impl Block for DeepslateDiamondOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:deepslate-diamond-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeepslateDiamondOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-deepslate-diamond-ore:block/deepslate_diamond_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeepslateDiamondOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeepslateDiamondOreBlock::RENDER;

pub struct BlockDeepslateDiamondOreMod;

impl BlockDeepslateDiamondOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
