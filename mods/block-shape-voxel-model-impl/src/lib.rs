use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::{BlockId, BlockManagerApi};
use block_shape_api::{BlockShape, BlockShapeApi, BlockShapeService};
use collision_api::Aabb;
use std::{collections::HashMap, marker::PhantomData, sync::Arc};
use tokio::task::JoinHandle;
use voxel_model_api::{VoxelModelApi, VoxelModelService};

pub struct BlockShapeVoxelModelImpl<B>(PhantomData<B>);

impl<B: BlockManagerApi> BlockShapeVoxelModelImpl<B> {
    pub fn init<M: VoxelModelApi>(bevy: &mut BevyMod, _blocks: &mut B, _models: &mut M) -> Self {
        let models = bevy.app.world().resource::<VoxelModelService>();
        let shapes = Arc::new(load_shapes::<B>(models));
        bevy.app
            .insert_resource(BlockShapeService::new(move |block| {
                shapes
                    .get(&block.block)
                    .cloned()
                    .unwrap_or_else(BlockShape::empty)
            }));
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl<B: BlockManagerApi> BlockShapeApi for BlockShapeVoxelModelImpl<B> {}

fn load_shapes<B: BlockManagerApi>(models: &VoxelModelService) -> HashMap<BlockId, BlockShape> {
    B::all()
        .iter()
        .copied()
        .map(|block| (block, load_shape::<B>(models, block)))
        .collect()
}

fn load_shape<B: BlockManagerApi>(models: &VoxelModelService, block: BlockId) -> BlockShape {
    if B::is_air(block) {
        return BlockShape::empty();
    }
    let Some(model) = B::render_info(block).model else {
        return BlockShape::full_cube();
    };
    match models.boxes(model) {
        Ok(boxes) if !boxes.is_empty() => BlockShape::new(
            boxes
                .iter()
                .map(|bounds| Aabb {
                    min: Vec3::from_array(bounds.min),
                    max: Vec3::from_array(bounds.max),
                })
                .collect::<Vec<_>>(),
        ),
        Ok(_) => BlockShape::full_cube(),
        Err(error) => {
            eprintln!("failed to load block shape for '{model}': {error}");
            BlockShape::full_cube()
        }
    }
}
