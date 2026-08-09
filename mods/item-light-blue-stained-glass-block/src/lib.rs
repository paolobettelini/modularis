use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LightBlueStainedGlassBlockItem;

impl Item for LightBlueStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:light_blue_stained_glass_block",
        label: "Light Blue Stained Glass",
    };
}

impl ItemRender for LightBlueStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-light-blue-stained-glass-block:item/light_blue_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LightBlueStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LightBlueStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemLightBlueStainedGlassBlockMod;

impl ItemLightBlueStainedGlassBlockMod {
    pub fn init(
        _block: &mut block_light_blue_stained_glass::BlockLightBlueStainedGlassMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
