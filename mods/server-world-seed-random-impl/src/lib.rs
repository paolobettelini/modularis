use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_world_seed_api::{ServerWorldSeed, ServerWorldSeedApi};
use std::hash::{BuildHasher, Hasher, RandomState};
use tokio::task::JoinHandle;

pub const WORLD_SEED_ENV: &str = "PATCHWORK_WORLD_SEED";

pub struct ServerWorldSeedRandomImpl;

impl ServerWorldSeedRandomImpl {
    pub fn init(bevy: &mut BevyMod) -> Self {
        let root = std::env::var(WORLD_SEED_ENV)
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or_else(random_seed);
        info!("server world seed: {root}");
        bevy.app.insert_resource(ServerWorldSeed::new(root));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerWorldSeedApi for ServerWorldSeedRandomImpl {}

fn random_seed() -> u64 {
    RandomState::new().build_hasher().finish()
}
