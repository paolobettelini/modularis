use entity_state_api::{EntitySnapshot,Uuid};
#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
pub struct EntityUpsertPacket {pub entity:EntitySnapshot,pub sequence:u64}
#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
pub struct EntityDespawnPacket {pub uuid:Uuid,pub sequence:u64}
