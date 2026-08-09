use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PotentSulfurBlockItem;

impl Item for PotentSulfurBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:potent_sulfur_block",
        label: "Potent Sulfur",
    };
}

impl ItemRender for PotentSulfurBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-potent-sulfur-block:item/potent_sulfur_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PotentSulfurBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PotentSulfurBlockItem as ItemRender>::RENDER;

pub struct ItemPotentSulfurBlockMod;

impl ItemPotentSulfurBlockMod {
    pub fn init(_block: &mut block_potent_sulfur::BlockPotentSulfurMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
