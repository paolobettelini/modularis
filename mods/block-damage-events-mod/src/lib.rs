use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_damage_events_api::ServerBlockDamageChanged;
use tokio::task::JoinHandle;

pub struct BlockDamageEventsMod;
impl BlockDamageEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self { bevy.app.add_message::<ServerBlockDamageChanged>(); Self }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

