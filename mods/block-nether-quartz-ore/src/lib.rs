use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct NetherQuartzOreBlock;

impl Block for NetherQuartzOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:nether-quartz-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for NetherQuartzOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-nether-quartz-ore:block/nether_quartz_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = NetherQuartzOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = NetherQuartzOreBlock::RENDER;

pub struct BlockNetherQuartzOreMod;

impl BlockNetherQuartzOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
