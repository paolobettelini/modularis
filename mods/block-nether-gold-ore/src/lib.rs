use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct NetherGoldOreBlock;

impl Block for NetherGoldOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:nether-gold-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for NetherGoldOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-nether-gold-ore:block/nether_gold_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = NetherGoldOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = NetherGoldOreBlock::RENDER;

pub struct BlockNetherGoldOreMod;

impl BlockNetherGoldOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
