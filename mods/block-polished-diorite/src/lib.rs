use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PolishedDioriteBlock;

impl Block for PolishedDioriteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:polished-diorite",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PolishedDioriteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-polished-diorite:block/polished_diorite"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PolishedDioriteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PolishedDioriteBlock::RENDER;

pub struct BlockPolishedDioriteMod;

impl BlockPolishedDioriteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
