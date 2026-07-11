#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Dimension {
    Aether,
    Nether,
    Overworld,
}

pub const ALL_DIMENSIONS: &[Dimension] = &[
    Dimension::Aether,
    Dimension::Nether,
    Dimension::Overworld,
];

pub fn all_dimensions() -> &'static [Dimension] { ALL_DIMENSIONS }

pub fn from_str(id: &str) -> Option<Dimension> {
    match id {
        "demo:aether" => Some(Dimension::Aether),
        "demo:nether" => Some(Dimension::Nether),
        "demo:overworld" => Some(Dimension::Overworld),
        _ => None,
    }
}

pub fn id(dimension: Dimension) -> &'static str {
    match dimension {
        Dimension::Aether => "demo:aether",
        Dimension::Nether => "demo:nether",
        Dimension::Overworld => "demo:overworld",
    }
}
