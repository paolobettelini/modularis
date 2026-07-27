use bevy::prelude::*;
use bevy_mod::BevyMod;
use serde::{Deserialize, Serialize};
use server_world_catalog_api::{ServerWorldCatalog, ServerWorldCatalogApi, WorldDirectory};
use server_world_seed_api::{ServerWorldSeed, ServerWorldSeedApi};
use std::{
    collections::HashMap,
    fs,
    hash::{BuildHasher, Hasher, RandomState},
    path::Path,
};
use tokio::task::JoinHandle;

pub const WORLD_SEED_ENV: &str = "PATCHWORK_WORLD_SEED";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct WorldInfo {
    seed: u64,
}

pub struct ServerWorldSeedCatalogFsImpl;

impl ServerWorldSeedCatalogFsImpl {
    pub fn init<C: ServerWorldCatalogApi>(bevy: &mut BevyMod, _catalog_api: &mut C) -> Self {
        let catalog = bevy.app.world().resource::<ServerWorldCatalog>().clone();
        let worlds = catalog.worlds();
        assert!(
            !worlds.is_empty(),
            "the filesystem seed provider requires at least one catalogued world"
        );
        let root_seed = configured_seed().unwrap_or_else(random_seed);
        let mut instance_seeds = HashMap::new();
        for world in worlds {
            let seed = load_or_create_world_info(&world, root_seed).unwrap_or_else(|error| {
                panic!(
                    "failed to initialize world '{}': {error}",
                    world.id.as_str()
                )
            });
            info!(
                "world '{}' ({}) seed: {seed}",
                world.id.as_str(),
                world.instance
            );
            instance_seeds.insert(world.instance, seed);
        }
        bevy.app
            .insert_resource(ServerWorldSeed::with_instance_seeds(
                root_seed,
                instance_seeds,
            ));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerWorldSeedApi for ServerWorldSeedCatalogFsImpl {}

fn load_or_create_world_info(world: &WorldDirectory, root_seed: u64) -> Result<u64, String> {
    fs::create_dir_all(world.root.join("data/chunk")).map_err(|error| error.to_string())?;
    let path = world.root.join("info.json");
    if path.exists() {
        let bytes = fs::read(&path).map_err(|error| error.to_string())?;
        let info: WorldInfo = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
        return Ok(info.seed);
    }
    let seed = ServerWorldSeed::new(root_seed).derive("patchwork:world", &world.instance);
    let bytes =
        serde_json::to_vec_pretty(&WorldInfo { seed }).map_err(|error| error.to_string())?;
    atomic_write(&path, &bytes)?;
    Ok(seed)
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, bytes).map_err(|error| error.to_string())?;
    fs::rename(&temporary, path).map_err(|error| error.to_string())
}

fn configured_seed() -> Option<u64> {
    std::env::var(WORLD_SEED_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
}

fn random_seed() -> u64 {
    RandomState::new().build_hasher().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use server_world_catalog_api::WorldId;
    use std::{
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };
    use world_instance_api::WorldInstanceId;

    #[test]
    fn world_info_keeps_the_seed_across_restarts() {
        let root = temporary_world_root();
        let world = WorldDirectory {
            id: WorldId::new("seed-test").unwrap(),
            instance: WorldInstanceId::new("test:seed"),
            root: root.clone(),
        };
        let first = load_or_create_world_info(&world, 42).unwrap();
        let second = load_or_create_world_info(&world, 999).unwrap();
        assert_eq!(first, second);
        let info: WorldInfo =
            serde_json::from_slice(&fs::read(root.join("info.json")).unwrap()).unwrap();
        assert_eq!(info.seed, first);
        fs::remove_dir_all(root).unwrap();
    }

    fn temporary_world_root() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "patchwork-world-seed-{}-{unique}",
            std::process::id()
        ))
    }
}
