use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct EmeraldOreBlockItem;

impl Item for EmeraldOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:emerald_ore_block",
        label: "Emerald Ore",
    };
}

impl ItemRender for EmeraldOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-emerald-ore-block:item/emerald_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = EmeraldOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <EmeraldOreBlockItem as ItemRender>::RENDER;

pub struct ItemEmeraldOreBlockMod;

impl ItemEmeraldOreBlockMod {
    pub fn init(_block: &mut block_emerald_ore::BlockEmeraldOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
