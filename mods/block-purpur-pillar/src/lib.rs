use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PurpurPillarBlock;

impl Block for PurpurPillarBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:purpur-pillar",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PurpurPillarBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-purpur-pillar:block/purpur_pillar"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PurpurPillarBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PurpurPillarBlock::RENDER;

pub struct BlockPurpurPillarMod;

impl BlockPurpurPillarMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
