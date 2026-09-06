use bevy::prelude::*;
use std::collections::HashMap;
use voxel_frame_api::*;
use world_instance_api::WorldScopeId;
use server_player_registry_api::PendingServerPlayerMove;
use server_chunk_world_api::ServerChunkWorld;
#[derive(Clone)]
pub struct ServerSurfaceAnchor{pub scope:WorldScopeId,pub frame:VoxelFrameId,pub local_foot:Vec3,pub world_foot:Vec3,pub epoch:u64}
#[derive(Resource,Default)]
pub struct ServerSurfaceAnchors(pub HashMap<u64,ServerSurfaceAnchor>);
/// Only a previously server-observed support can rebase movement. Speed and
/// collision validation still run afterwards on the relative displacement.
pub fn rebase(world:&ServerChunkWorld,anchor:&ServerSurfaceAnchor,movement:&mut PendingServerPlayerMove){
 let Some(claim)=movement.surface else{return;};
 let foot=Vec3::from_array(claim.local_foot);
 if !foot.is_finite()||anchor.epoch!=movement.movement_epoch||claim.surface!=anchor.frame.0.as_u128()||movement.current_position.distance_squared(anchor.world_foot)>0.0001{return;}
 let Some(pose)=world.frames().transform(&anchor.scope,anchor.frame)else{return;};
 let current=pose.local_to_world(anchor.local_foot.as_dvec3()).as_vec3();
 let requested=pose.local_to_world(foot.as_dvec3()).as_vec3();
 if !current.is_finite()||!requested.is_finite(){return;}
 movement.current_position=current;
 movement.requested_position=requested;
 movement.accepted_position=movement.requested_position;
}
