#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockInfo {
    pub id: &'static str,
    pub is_air: bool,
    pub solid: bool,
    pub opaque: bool,
}

pub trait Block {
    const INFO: BlockInfo;
}
