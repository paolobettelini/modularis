use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MagentaStainedGlassBlockItem;

impl Item for MagentaStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:magenta_stained_glass_block",
        label: "Magenta Stained Glass",
    };
}

impl ItemRender for MagentaStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-magenta-stained-glass-block:item/magenta_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MagentaStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MagentaStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemMagentaStainedGlassBlockMod;

impl ItemMagentaStainedGlassBlockMod {
    pub fn init(_block: &mut block_magenta_stained_glass::BlockMagentaStainedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
