use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PurpleWoolBlock;

impl Block for PurpleWoolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:purple-wool",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PurpleWoolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-purple-wool:block/purple_wool"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PurpleWoolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PurpleWoolBlock::RENDER;

pub struct BlockPurpleWoolMod;

impl BlockPurpleWoolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
