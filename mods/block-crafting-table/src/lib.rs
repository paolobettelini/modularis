use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CraftingTableBlock;

impl Block for CraftingTableBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:crafting-table",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CraftingTableBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-crafting-table:block/crafting_table"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CraftingTableBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CraftingTableBlock::RENDER;

pub struct BlockCraftingTableMod;

impl BlockCraftingTableMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
