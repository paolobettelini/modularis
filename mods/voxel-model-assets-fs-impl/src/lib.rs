use bevy_mod::BevyMod;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::task::JoinHandle;
use voxel_model_api::{VoxelModelApi, VoxelModelData, VoxelModelLoadResult, VoxelModelService};
use voxel_models_lib::{
    BakeOptions, ModAssetsResourcePack, ModelResolver, ResourceLocation, bake_model,
    bake_model_boxes,
};

pub struct VoxelModelAssetsFsImpl;

impl VoxelModelAssetsFsImpl {
    pub fn init(bevy: &mut BevyMod) -> Self {
        let source = Arc::new(ModAssetsResourcePack::new("assets"));
        let cache = Arc::new(Mutex::new(HashMap::<String, VoxelModelLoadResult>::new()));
        bevy.app.insert_resource(VoxelModelService::new(move |id| {
            if let Some(result) = cache.lock().expect("model cache poisoned").get(id).cloned() {
                return result;
            }

            let result = (|| {
                let location = ResourceLocation::parse(id)
                    .map_err(|error| Arc::<str>::from(error.to_string()))?;
                let resolved = ModelResolver::new(source.as_ref())
                    .resolve(&location)
                    .map_err(|error| Arc::<str>::from(error.to_string()))?;
                let options = BakeOptions::default();
                let quads = bake_model(&resolved, &options)
                    .map_err(|error| Arc::<str>::from(error.to_string()))?;
                let boxes = bake_model_boxes(&resolved, &options);
                Ok(Arc::new(VoxelModelData {
                    quads: quads.into(),
                    boxes: boxes.into(),
                }))
            })();
            cache
                .lock()
                .expect("model cache poisoned")
                .insert(id.to_string(), result.clone());
            result
        }));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl VoxelModelApi for VoxelModelAssetsFsImpl {}
