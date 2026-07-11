use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_instance_api::BlockInstance;
use block_manager_api::BlockManagerApi;
use client_chunk_cache_api::{ClientChunkCache, ClientChunkCacheApi};
use collision_api::{CollisionApi, CollisionService};
use generated_block_registry::BlockId;
use player_block_collision_api::{collides_at as player_collides_at, resolve_player_collision};
use std::marker::PhantomData;
use tokio::task::JoinHandle;
use voxel_math_api::BlockPos;

pub struct BlockAabbCollisionImpl<B>(PhantomData<B>);

impl<B: BlockManagerApi> BlockAabbCollisionImpl<B> {
    pub fn init<C: ClientChunkCacheApi>(
        bevy: &mut BevyMod,
        _cache_api: &mut C,
        _blocks: &mut B,
    ) -> Self {
        let cache = bevy.app.world().resource::<ClientChunkCache>().clone();
        let collision_cache = cache.clone();
        let resolve_cache = cache;
        bevy.app.insert_resource(CollisionService::new(
            move |position, radius, height| {
                player_collides_at(position, radius, height, &|position| {
                    solid_block::<B>(&collision_cache, position)
                })
            },
            move |position, movement, radius, height| {
                resolve_player_collision(position, movement, radius, height, &|position| {
                    solid_block::<B>(&resolve_cache, position)
                })
            },
        ));
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl<B: BlockManagerApi> CollisionApi for BlockAabbCollisionImpl<B> {}

fn solid_block<B: BlockManagerApi>(cache: &ClientChunkCache, position: BlockPos) -> bool {
    let block = cache.block(position).unwrap_or_else(|| {
        if position.y <= 0 {
            BlockInstance::new(BlockId::Stone)
        } else {
            BlockInstance::new(BlockId::Air)
        }
    });
    B::is_solid(block.block)
}
