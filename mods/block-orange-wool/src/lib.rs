use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OrangeWoolBlock;

impl Block for OrangeWoolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:orange-wool",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OrangeWoolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-orange-wool:block/orange_wool"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OrangeWoolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OrangeWoolBlock::RENDER;

pub struct BlockOrangeWoolMod;

impl BlockOrangeWoolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
