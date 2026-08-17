use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WebPacket {
    GenerateAuthToken { player_uuid: Uuid },
    ServeAuthToken { token: String },
}
