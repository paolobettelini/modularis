#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemInfo {
    pub id: &'static str,
    pub label: &'static str,
}

pub trait Item {
    const INFO: ItemInfo;
}
