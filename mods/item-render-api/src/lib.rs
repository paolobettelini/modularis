#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemRenderInfo {
    /// Namespaced JSON model ID, for example `item-stick:item/stick`.
    pub model: Option<&'static str>,
}

pub trait ItemRender {
    const RENDER: ItemRenderInfo;
}
