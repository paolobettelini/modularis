use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MelonBlock;

impl Block for MelonBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:melon",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MelonBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-melon:block/melon"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MelonBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MelonBlock::RENDER;

pub struct BlockMelonMod;

impl BlockMelonMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
