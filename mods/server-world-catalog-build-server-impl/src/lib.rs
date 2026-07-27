use bevy_mod::BevyMod;
use generated_dimension_registry::{Dimension, id};
use server_world_catalog_api::{
    ServerWorldCatalog, ServerWorldCatalogApi, WorldDirectory, WorldId,
};
use std::path::PathBuf;
use tokio::task::JoinHandle;
use world_instance_api::WorldInstanceId;

pub struct ServerWorldCatalogBuildServerImpl;

impl ServerWorldCatalogBuildServerImpl {
    pub fn init(
        bevy: &mut BevyMod,
        _overworld: &mut dimension_overworld::DimensionOverworldMod,
        _nether: &mut dimension_nether::DimensionNetherMod,
        _aether: &mut dimension_aether::DimensionAetherMod,
    ) -> Self {
        let catalog = ServerWorldCatalog::default();
        let worlds_root = demo_root().join("build-server").join("worlds");
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
    catalog
        .register(WorldDirectory {
            id: WorldId::new(world_id).expect("demo world ids must be valid folder names"),
            instance: WorldInstanceId::new(id(dimension)),
            root: worlds_root.join(world_id),
        })
        .expect("demo world folders and instances must be unique");
}

fn demo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("the catalog mod must live under <demo>/mods/<mod>")
        .to_path_buf()
}
