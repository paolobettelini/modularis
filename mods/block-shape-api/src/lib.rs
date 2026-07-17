use bevy::prelude::*;
use block_instance_api::BlockInstance;
use collision_api::Aabb;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockShape(Arc<[Aabb]>);

impl BlockShape {
    pub fn new(boxes: impl Into<Arc<[Aabb]>>) -> Self {
        Self(boxes.into())
    }

    pub fn empty() -> Self {
        Self(Arc::from([]))
    }

    pub fn full_cube() -> Self {
        Self::new([Aabb {
            min: Vec3::ZERO,
            max: Vec3::ONE,
        }])
    }

    pub fn boxes(&self) -> &[Aabb] {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<[Aabb]> for BlockShape {
    fn as_ref(&self) -> &[Aabb] {
        self.boxes()
    }
}

type BlockShapeFn = dyn Fn(&BlockInstance) -> BlockShape + Send + Sync + 'static;

#[derive(Resource, Clone)]
pub struct BlockShapeService {
    shape: Arc<BlockShapeFn>,
}

impl BlockShapeService {
    pub fn new(shape: impl Fn(&BlockInstance) -> BlockShape + Send + Sync + 'static) -> Self {
        Self {
            shape: Arc::new(shape),
        }
    }

    pub fn shape(&self, block: &BlockInstance) -> BlockShape {
        (self.shape)(block)
    }
}

pub trait BlockShapeApi: Send + Sync + 'static {}
