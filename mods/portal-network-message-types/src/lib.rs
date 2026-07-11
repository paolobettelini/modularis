use generated_dimension_registry::Dimension;
use portal_api::PortalFrame;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortalOpenedPacket {
    pub frame: PortalFrame,
    pub destination: Dimension,
    pub color: [f32; 4],
}
