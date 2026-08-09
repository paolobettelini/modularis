use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SeaLanternBlock;

impl Block for SeaLanternBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:sea-lantern",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SeaLanternBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-sea-lantern:block/sea_lantern"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SeaLanternBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SeaLanternBlock::RENDER;

pub struct BlockSeaLanternMod;

impl BlockSeaLanternMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
