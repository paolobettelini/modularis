use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct NetherGoldOreBlockItem;

impl Item for NetherGoldOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:nether_gold_ore_block",
        label: "Nether Gold Ore",
    };
}

impl ItemRender for NetherGoldOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-nether-gold-ore-block:item/nether_gold_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = NetherGoldOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <NetherGoldOreBlockItem as ItemRender>::RENDER;

pub struct ItemNetherGoldOreBlockMod;

impl ItemNetherGoldOreBlockMod {
    pub fn init(_block: &mut block_nether_gold_ore::BlockNetherGoldOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
