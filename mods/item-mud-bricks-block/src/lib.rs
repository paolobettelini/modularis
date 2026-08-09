use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MudBricksBlockItem;

impl Item for MudBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:mud_bricks_block",
        label: "Mud Bricks",
    };
}

impl ItemRender for MudBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-mud-bricks-block:item/mud_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MudBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MudBricksBlockItem as ItemRender>::RENDER;

pub struct ItemMudBricksBlockMod;

impl ItemMudBricksBlockMod {
    pub fn init(_block: &mut block_mud_bricks::BlockMudBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
