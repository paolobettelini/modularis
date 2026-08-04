use bevy_mod::BevyMod;
use network_frame_security_api::{ClientFrameSecurity, ServerFrameSecurity};
use tokio::task::JoinHandle;

pub struct NetworkFrameSecurityStateMod;

impl NetworkFrameSecurityStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .init_resource::<ClientFrameSecurity>()
            .init_resource::<ServerFrameSecurity>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
