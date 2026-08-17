use bevy_mod::BevyMod;
use thecrown_game_config_api::{TheCrownGameConfig, TheCrownGameConfigApi};
use tokio::task::JoinHandle;
use uuid::Uuid;

pub struct TheCrownGameConfigLocalImpl;

impl TheCrownGameConfigLocalImpl {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.insert_resource(TheCrownGameConfig {
            server_id: format!("modularis-local-{}", Uuid::new_v4()),
            public_address: "127.0.0.1".to_owned(),
            public_port: 10000,
            nats_url: "nats://127.0.0.1:4222".to_owned(),
            request_timeout_ms: 2_000,
            web_address: "127.0.0.1:8080".to_owned(),
        });
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl TheCrownGameConfigApi for TheCrownGameConfigLocalImpl {}

