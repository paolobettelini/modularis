use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PackedMudBlock;

impl Block for PackedMudBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:packed-mud",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PackedMudBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-packed-mud:block/packed_mud"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PackedMudBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PackedMudBlock::RENDER;

pub struct BlockPackedMudMod;

impl BlockPackedMudMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
