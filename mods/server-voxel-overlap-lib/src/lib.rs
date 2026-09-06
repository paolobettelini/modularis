use bevy::math::DVec3;
use server_chunk_world_api::ServerChunkWorld;
use server_block_placement_api::PendingBlockPlacement;
use block_shape_api::BlockShapeService;
use voxel_frame_api::*;
use voxel_frame_geometry_lib::{OrientedVoxelBox,block_bounds};
use voxel_math_api::BlockPos;

/// Optional policy mechanism; neither storage nor frame creation forbid overlap.
pub fn overlaps_other_frame(world:&ServerChunkWorld,shapes:&BlockShapeService,intent:&PendingBlockPlacement)->bool {
    let Some(key)=world.resident_key_for_player(intent.item_use.player_id,intent.position.chunk()) else {return true;};
    let frames=world.frames();
    let Some(pose)=frames.transform(&key.scope(),intent.position.frame) else {return true;};
    for bounds in shapes.shape(&intent.block).boxes() {
        let local=block_bounds(intent.position.local,*bounds);
        let world_bounds=pose.transform_bounds(local);
        let target=OrientedVoxelBox::new(local,pose);
        let mut candidates=frames.query(&key.scope(),world_bounds).into_iter().map(|frame|(frame.id,frame.transform)).collect::<Vec<_>>();
        candidates.push((VoxelFrameId::ROOT,VoxelFrameTransform::IDENTITY));
        for (id,other_pose) in candidates {
            if id==intent.position.frame {continue;}
            let inverse=VoxelFrameTransform::new(
                other_pose.world_to_local(DVec3::ZERO).to_array(),other_pose.rotation().conjugate().to_array()).unwrap();
            let local_query=inverse.transform_bounds(world_bounds);
            let min=DVec3::from_array(local_query.min).floor();
            let max=DVec3::from_array(local_query.max).ceil();
            for y in min.y as i32..max.y as i32 { for z in min.z as i32..max.z as i32 { for x in min.x as i32..max.x as i32 {
                let address=VoxelBlockAddress::new(id,BlockPos::new(x,y,z));
                if !id.is_root() && !frames.get(&key.scope(),id).is_some_and(|f|f.occupied_chunks.contains(&address.chunk().local)) {continue;}
                let Some(state)=world.block_for_player(intent.item_use.player_id,address) else {continue;};
                if shapes.shape(&state).boxes().iter().any(|b|target.overlaps(OrientedVoxelBox::new(block_bounds(address.local,*b),other_pose))) {return true;}
            }}}
        }
    }
    false
}

/// Check reservations from earlier accepted requests in the same update.
/// This closes the validate-all/apply-all gap without putting policy in storage.
pub fn placements_overlap(world:&ServerChunkWorld,shapes:&BlockShapeService,a:&PendingBlockPlacement,b:&PendingBlockPlacement)->bool {
    if a.position.frame==b.position.frame {return false;}
    let Some(ka)=world.resident_key_for_player(a.item_use.player_id,a.position.chunk()) else {return true;};
    let Some(kb)=world.resident_key_for_player(b.item_use.player_id,b.position.chunk()) else {return true;};
    if ka.scope()!=kb.scope() {return false;}
    let frames=world.frames();
    let Some(pa)=frames.transform(&ka.scope(),a.position.frame) else {return true;};
    let Some(pb)=frames.transform(&kb.scope(),b.position.frame) else {return true;};
    shapes.shape(&a.block).boxes().iter().any(|ba|shapes.shape(&b.block).boxes().iter().any(|bb|
        OrientedVoxelBox::new(block_bounds(a.position.local,*ba),pa).overlaps(OrientedVoxelBox::new(block_bounds(b.position.local,*bb),pb))))
}
