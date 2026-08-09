use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeepslateLapisOreBlock;

impl Block for DeepslateLapisOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:deepslate-lapis-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeepslateLapisOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-deepslate-lapis-ore:block/deepslate_lapis_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeepslateLapisOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeepslateLapisOreBlock::RENDER;

pub struct BlockDeepslateLapisOreMod;

impl BlockDeepslateLapisOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
