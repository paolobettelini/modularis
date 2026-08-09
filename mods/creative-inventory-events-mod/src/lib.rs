use bevy_mod::BevyMod;
use creative_inventory_events_api::{CreativeItemTakeRequested, LocalCreativeItemTakeIntent};
use tokio::task::JoinHandle;

pub struct CreativeInventoryEventsMod;

impl CreativeInventoryEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .add_message::<LocalCreativeItemTakeIntent>()
            .add_message::<CreativeItemTakeRequested>();
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}
