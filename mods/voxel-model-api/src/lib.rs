use bevy::prelude::*;
use std::sync::Arc;
use voxel_models_lib::{BakedModelBox, BakedQuad, ResourceLocation};

pub type SharedBakedModel = Arc<[BakedQuad]>;
pub type SharedModelBoxes = Arc<[BakedModelBox]>;

#[derive(Debug, Clone)]
pub struct VoxelModelData {
    pub quads: SharedBakedModel,
    pub boxes: SharedModelBoxes,
}

pub type SharedVoxelModelData = Arc<VoxelModelData>;
pub type VoxelModelLoadResult = Result<SharedVoxelModelData, Arc<str>>;

type LoadModelFn = dyn Fn(&str) -> VoxelModelLoadResult + Send + Sync + 'static;

#[derive(Resource, Clone)]
pub struct VoxelModelService {
    load_model: Arc<LoadModelFn>,
}

impl VoxelModelService {
    pub fn new(load_model: impl Fn(&str) -> VoxelModelLoadResult + Send + Sync + 'static) -> Self {
        Self {
            load_model: Arc::new(load_model),
        }
    }

    pub fn load(&self, model: &str) -> VoxelModelLoadResult {
        (self.load_model)(model)
    }

    pub fn bake(&self, model: &str) -> Result<SharedBakedModel, Arc<str>> {
        self.quads(model)
    }

    pub fn quads(&self, model: &str) -> Result<SharedBakedModel, Arc<str>> {
        self.load(model).map(|data| data.quads.clone())
    }

    pub fn boxes(&self, model: &str) -> Result<SharedModelBoxes, Arc<str>> {
        self.load(model).map(|data| data.boxes.clone())
    }

    pub fn texture_asset_path(texture: &ResourceLocation) -> String {
        format!("{}/textures/{}.png", texture.namespace(), texture.path())
    }
}

pub trait VoxelModelApi: Send + Sync + 'static {}
