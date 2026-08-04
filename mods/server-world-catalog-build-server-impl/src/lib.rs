use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_dimension_registry::{Dimension, id};
use server_world_catalog_api::{
    ServerWorldCatalog, ServerWorldCatalogApi, WorldDirectory, WorldId,
};
use std::{fs, path::PathBuf};
use tokio::task::JoinHandle;
use world_instance_api::WorldInstanceId;

pub struct ServerWorldCatalogBuildServerImpl;

impl ServerWorldCatalogBuildServerImpl {
    pub fn init(
        bevy: &mut BevyMod,
        _overworld: &mut dimension_overworld::DimensionOverworldMod,
        _nether: &mut dimension_nether::DimensionNetherMod,
        _aether: &mut dimension_aether::DimensionAetherMod,
        _logging: &mut server_bevy_log_mod::ServerBevyLogMod,
    ) -> Self {
        let catalog = ServerWorldCatalog::default();
        let data_root = executable_data_root();
        let worlds_root = data_root.join("worlds");
        fs::create_dir_all(&worlds_root).unwrap_or_else(|error| {
            panic!(
                "failed to create server worlds directory '{}': {error}",
                worlds_root.display()
            )
        });
        info!("server data directory: {}", data_root.display());
        info!("server worlds directory: {}", worlds_root.display());
        register(&catalog, &worlds_root, "overworld", Dimension::Overworld);
        register(&catalog, &worlds_root, "nether", Dimension::Nether);
        register(&catalog, &worlds_root, "aether", Dimension::Aether);
        bevy.app.insert_resource(catalog);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerWorldCatalogApi for ServerWorldCatalogBuildServerImpl {}

fn register(
    catalog: &ServerWorldCatalog,
    worlds_root: &std::path::Path,
    world_id: &str,
    dimension: Dimension,
) {
    let root = worlds_root.join(world_id);
    let instance = WorldInstanceId::new(id(dimension));
    info!(
        "registered world '{world_id}' as instance '{instance}' at {}",
        root.display()
    );
    catalog
        .register(WorldDirectory {
            id: WorldId::new(world_id).expect("demo world ids must be valid folder names"),
            instance,
            root,
        })
        .expect("demo world folders and instances must be unique");
}

fn executable_data_root() -> PathBuf {
    let executable = std::env::current_exe()
        .unwrap_or_else(|error| panic!("failed to resolve the server executable path: {error}"));
    executable
        .parent()
        .unwrap_or_else(|| {
            panic!(
                "server executable path '{}' has no parent directory",
                executable.display()
            )
        })
        .join("data")
}
