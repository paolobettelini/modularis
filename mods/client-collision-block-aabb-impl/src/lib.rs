use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use block_shape_api::{BlockShape, BlockShapeApi, BlockShapeService};
use client_chunk_cache_api::{ClientChunkCache, ClientChunkCacheApi};
use collision_api::{CollisionApi, CollisionService};
use std::marker::PhantomData;
use tokio::task::JoinHandle;

pub struct BlockAabbCollisionImpl<B>(PhantomData<B>);

impl<B: BlockManagerApi> BlockAabbCollisionImpl<B> {
    pub fn init<C: ClientChunkCacheApi, S: BlockShapeApi>(
        bevy: &mut BevyMod,
        _cache_api: &mut C,
        _blocks: &mut B,
        _shapes: &mut S,
    ) -> Self {
        bevy.app.init_resource::<client_voxel_frame_api::ClientVoxelFrames>();
        let backend=FrameCollision::<B>{
            cache:bevy.app.world().resource::<ClientChunkCache>().clone(),
            shapes:bevy.app.world().resource::<BlockShapeService>().clone(),
            frames:bevy.app.world().resource::<client_voxel_frame_api::ClientVoxelFrames>().registry.clone(),
            marker:PhantomData,
        };
        bevy.app.insert_resource(CollisionService::new(|_,_,_|false,|position,movement,_,_|collision_api::CollisionResult{position:position+movement,grounded:false,hit_x:false,hit_y:false,hit_z:false}).with_character_backend(backend));
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl<B: BlockManagerApi> CollisionApi for BlockAabbCollisionImpl<B> {}

struct FrameCollision<B>{cache:ClientChunkCache,shapes:BlockShapeService,frames:voxel_frame_registry_api::VoxelFrames,marker:PhantomData<B>}
impl<B:BlockManagerApi> FrameCollision<B>{
 fn geometry(&self)->impl collision_api::CharacterGeometry+'_{
  voxel_frame_collision_lib::VoxelGeometry{
   shape:|address:voxel_frame_api::VoxelBlockAddress|{
    // Unloaded data blocks traversal on every axis; no invented global Y floor.
    self.cache.block(address).map_or_else(BlockShape::full_cube,|block|if B::is_solid(block.block){self.shapes.shape(&block)}else{BlockShape::empty()})
   },
   frames:|bounds|self.frames.scopes().iter().flat_map(|scope|self.frames.query(scope,bounds)).collect(),
  }
 }
}
impl<B:BlockManagerApi> collision_api::CharacterCollisionBackend for FrameCollision<B>{
 fn resolve(&self,q:collision_api::CharacterQuery)->collision_api::CharacterResult{character_collision_lib::resolve(q,&self.geometry())}
 fn support(&self,q:collision_api::CharacterQuery)->Option<collision_api::CharacterContact>{character_collision_lib::support(q,&self.geometry())}
 fn surface_pose(&self,id:u128)->Option<(Vec3,Quat)>{
  if id==0{return Some((Vec3::ZERO,Quat::IDENTITY));}
  for scope in self.frames.scopes(){
   if let Some(frame)=self.frames.get(&scope,voxel_frame_api::VoxelFrameId::from_u128(id)){
    return Some((frame.transform.translation().as_vec3(),frame.transform.rotation().as_quat()));
   }
  }None
 }
}
