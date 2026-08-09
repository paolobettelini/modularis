use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeepslateCopperOreBlock;

impl Block for DeepslateCopperOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:deepslate-copper-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeepslateCopperOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-deepslate-copper-ore:block/deepslate_copper_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeepslateCopperOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeepslateCopperOreBlock::RENDER;

pub struct BlockDeepslateCopperOreMod;

impl BlockDeepslateCopperOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
