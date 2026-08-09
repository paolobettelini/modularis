use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MudBlockItem;

impl Item for MudBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:mud_block",
        label: "Mud",
    };
}

impl ItemRender for MudBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-mud-block:item/mud_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MudBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MudBlockItem as ItemRender>::RENDER;

pub struct ItemMudBlockMod;

impl ItemMudBlockMod {
    pub fn init(_block: &mut block_mud::BlockMudMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
