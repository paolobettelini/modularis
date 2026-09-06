use bevy::prelude::*;
use server_chunk_world_api::ResidentChunkKey;

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerChunkSent { pub player_id: u64, pub key: ResidentChunkKey }


/// Request/load work policy, independent of world storage and interest shape.
#[derive(Resource,Debug,Clone,Copy)]
pub struct ServerChunkRequestBudget {
    pub chunks_per_update: usize,
    pub max_pending_per_player: usize,
    pub max_pending_total: usize,
    pub time_budget_millis: u64,
}
impl Default for ServerChunkRequestBudget {
    fn default()->Self {Self{chunks_per_update:32,max_pending_per_player:256,max_pending_total:4096,time_budget_millis:8}}
}
