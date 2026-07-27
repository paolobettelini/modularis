use bevy::prelude::*;
use client_dimension_api::Dimension;
use generated_block_registry::BlockId;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrassTintContext {
    pub dimension: Dimension,
    pub support: BlockId,
}

pub type GrassTintFunction = dyn Fn(GrassTintContext) -> Vec3 + Send + Sync + 'static;

#[derive(Resource, Clone)]
pub struct GrassTintService {
    tint: Arc<GrassTintFunction>,
}

impl GrassTintService {
    pub fn new(tint: impl Fn(GrassTintContext) -> Vec3 + Send + Sync + 'static) -> Self {
        Self {
            tint: Arc::new(tint),
        }
    }

    pub fn tint(&self, context: GrassTintContext) -> Vec3 {
        (self.tint)(context)
    }
}

pub trait ClientGrassTintApi: Send + Sync + 'static {}
