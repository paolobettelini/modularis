use bevy::prelude::*;
use std::sync::Arc;
use voxel_models_lib::{BakedQuad, ResourceLocation};

pub type SharedBakedModel = Arc<[BakedQuad]>;
pub type VoxelModelLoadResult = Result<SharedBakedModel, Arc<str>>;

type BakeModelFn = dyn Fn(&str) -> VoxelModelLoadResult + Send + Sync + 'static;

#[derive(Resource, Clone)]
pub struct VoxelModelService {
    bake_model: Arc<BakeModelFn>,
}

impl VoxelModelService {
    pub fn new(bake_model: impl Fn(&str) -> VoxelModelLoadResult + Send + Sync + 'static) -> Self {
        Self {
            bake_model: Arc::new(bake_model),
        }
    }

    pub fn bake(&self, model: &str) -> VoxelModelLoadResult {
        (self.bake_model)(model)
    }

    pub fn texture_asset_path(texture: &ResourceLocation) -> String {
        format!("{}/textures/{}.png", texture.namespace(), texture.path())
    }
}

pub trait VoxelModelApi: Send + Sync + 'static {}
