use bevy_mod::BevyMod;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::task::JoinHandle;
use voxel_model_api::{VoxelModelApi, VoxelModelLoadResult, VoxelModelService};
use voxel_models_lib::{
    BakeOptions, ModAssetsResourcePack, ModelResolver, ResourceLocation, bake_model,
};

pub struct ClientVoxelModelAssetsFsImpl;

impl ClientVoxelModelAssetsFsImpl {
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
                Ok(bake_model(&resolved, &BakeOptions::default())
                    .map_err(|error| Arc::<str>::from(error.to_string()))?
                    .into())
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

impl VoxelModelApi for ClientVoxelModelAssetsFsImpl {}
