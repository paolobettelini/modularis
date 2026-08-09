use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct IronOreBlock;

impl Block for IronOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:iron-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for IronOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-iron-ore:block/iron_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = IronOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = IronOreBlock::RENDER;

pub struct BlockIronOreMod;

impl BlockIronOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
