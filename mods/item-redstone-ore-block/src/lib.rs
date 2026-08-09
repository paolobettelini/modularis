use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct RedstoneOreBlockItem;

impl Item for RedstoneOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:redstone_ore_block",
        label: "Redstone Ore",
    };
}

impl ItemRender for RedstoneOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-redstone-ore-block:item/redstone_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = RedstoneOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RedstoneOreBlockItem as ItemRender>::RENDER;

pub struct ItemRedstoneOreBlockMod;

impl ItemRedstoneOreBlockMod {
    pub fn init(_block: &mut block_redstone_ore::BlockRedstoneOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
