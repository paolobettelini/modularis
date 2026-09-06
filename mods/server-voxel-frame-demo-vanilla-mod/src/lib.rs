//! Optional demonstration policy, not a generator or a special chunk format.
use bevy::{math::DQuat,prelude::*};
use bevy_mod::BevyMod;
use server_chunk_world_api::{ServerChunkWorld,ServerChunkWorldApi};
use server_chunk_provider_api::ChunkViewer;
use voxel_frame_api::*;
use voxel_math_api::{BlockPos,ChunkPos};
use generated_block_registry::BlockId;
use tokio::task::JoinHandle;
pub struct ServerVoxelFrameDemoVanillaMod;
impl ServerVoxelFrameDemoVanillaMod {
    pub fn init<W:ServerChunkWorldApi>(
        bevy:&mut BevyMod,_world:&mut W,_persistence:&mut server_voxel_frame_persistence_mod::ServerVoxelFramePersistenceMod,
        _stone:&mut block_stone::BlockStoneMod,_stairs:&mut block_oak_stairs::BlockOakStairsMod,_table:&mut block_crafting_table::BlockCraftingTableMod,
    )->Self {
        bevy.app.add_systems(Startup,create_demo_frame);
        Self
    }
    pub fn run(&self)->Option<Vec<JoinHandle<()>>>{None}
}
fn create_demo_frame(world:Res<ServerChunkWorld>) {
    let viewer=ChunkViewer::Server;
    let Some(key)=world.resident_key(viewer,ChunkPos::new(0,0,0)) else {warn!("demo frame: no default world route");return;};
    let id=VoxelFrameId(uuid::Uuid::from_u128(0x952d5d39a38b4c99af5b84b3f139cd42));
    if world.frames().get(&key.scope(),id).is_some() { info!("restored demo voxel frame {id}"); return; }
    // The sample stays above nearby terrain, but its geometry is entirely local.
    let ground=(-32..128).rev().find(|y|world.block_for(viewer,BlockPos::new(10,*y,0)).is_some_and(|b|b.block!=BlockId::Air)).unwrap_or(2);
    let pose=VoxelFrameTransform::new([10.0,ground as f64+4.0,0.0],DQuat::from_euler(EulerRot::XYZ,0.13,0.37,-0.19).to_array()).unwrap();
    world.frames().upsert(VoxelFrame::new(id,key.scope(),pose)).unwrap();
    let set=|x,y,z,block|world.set_block_for(viewer,VoxelBlockAddress::new(id,BlockPos::new(x,y,z)),block).expect("cannot build demo frame");
    for z in -3..=3 { for x in -3..=3 { set(x,0,z,BlockId::Stone); } }
    set(-2,1,1,BlockId::OakStairs);
    set(-2,1,2,BlockId::Stone);
    set(-2,2,2,BlockId::OakStairs);
    set(2,1,1,BlockId::CraftingTable);
    info!("created tilted demo voxel frame {id} near [10, {}, 0]",ground+4);
}
