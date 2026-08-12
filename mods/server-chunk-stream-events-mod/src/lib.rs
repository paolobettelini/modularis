use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_chunk_stream_events_api::ServerChunkSent;
use tokio::task::JoinHandle;

pub struct ServerChunkStreamEventsMod;
impl ServerChunkStreamEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self { bevy.app.add_message::<ServerChunkSent>(); Self }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

