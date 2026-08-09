use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BlueWoolBlock;

impl Block for BlueWoolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:blue-wool",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BlueWoolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-blue-wool:block/blue_wool"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BlueWoolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BlueWoolBlock::RENDER;

pub struct BlockBlueWoolMod;

impl BlockBlueWoolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
