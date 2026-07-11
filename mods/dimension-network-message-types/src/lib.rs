use generated_dimension_registry::Dimension;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerDimensionChanged {
    pub dimension: Dimension,
    pub position: [f32; 3],
}
