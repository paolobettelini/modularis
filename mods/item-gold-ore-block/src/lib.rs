use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GoldOreBlockItem;

impl Item for GoldOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:gold_ore_block",
        label: "Gold Ore",
    };
}

impl ItemRender for GoldOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-gold-ore-block:item/gold_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GoldOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GoldOreBlockItem as ItemRender>::RENDER;

pub struct ItemGoldOreBlockMod;

impl ItemGoldOreBlockMod {
    pub fn init(_block: &mut block_gold_ore::BlockGoldOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
