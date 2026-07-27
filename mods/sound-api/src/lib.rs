#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoundInfo {
    pub id: &'static str,
    pub asset_path: &'static str,
}

pub trait Sound {
    const INFO: SoundInfo;
}
