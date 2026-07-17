use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct WarpedNyliumBlock;
impl Block for WarpedNyliumBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:warped-nylium",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for WarpedNyliumBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-warped-nylium:block/warped_nylium"),
        textures: None,
    };
}
pub const BLOCK_INFO: BlockInfo = WarpedNyliumBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WarpedNyliumBlock::RENDER;
pub struct BlockWarpedNyliumMod;
impl BlockWarpedNyliumMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
