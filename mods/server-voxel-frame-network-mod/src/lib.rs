use bevy::{math::DVec3,prelude::*};
use bevy_mod::BevyMod;
use server_chunk_world_api::{ServerChunkWorld,ServerChunkWorldApi};
use server_player_registry_api::{ServerPlayerRegistry,ServerPlayerRegistryApi,ServerPlayerMovementSet,ServerPlayerRelocationSet};
use server_chunk_residency_api::{ServerChunkResidencyApi,ServerChunkResidencyConfig};
use server_network_events_api::{ServerNetworkEventsApi,ServerPacketOut,ServerAudience};
use network_protocol_mod::NetworkProtocolMod;
use generated_network_messages::ClientBoundMessage;
use voxel_frame_api::*;
use voxel_frame_registry_api::VoxelFrameSet;
use voxel_frame_network_message_types::*;
use voxel_math_api::BlockPos;
use std::collections::{HashMap,HashSet};
use tokio::task::JoinHandle;
#[derive(Resource,Default)]
struct ReplicatedMotions(HashMap<(u64,VoxelFrameId),voxel_frame_movement_lib::FrameTrajectory>);
#[derive(Resource,Default)]
struct ReplicatedFrames(HashMap<u64,(u64,u64,HashMap<VoxelFrameId,VoxelFrame>)>);
pub struct ServerVoxelFrameNetworkMod;
impl ServerVoxelFrameNetworkMod {
    pub fn init<W:ServerChunkWorldApi,P:ServerPlayerRegistryApi,R:ServerChunkResidencyApi,N:ServerNetworkEventsApi>(
        bevy:&mut BevyMod,_world:&mut W,_players:&mut P,_residency:&mut R,_network:&mut N,_protocol:&mut NetworkProtocolMod
    )->Self {
        bevy.app.add_message::<server_voxel_frame_motion_api::SendVoxelFrameMotion>().init_resource::<ReplicatedFrames>().init_resource::<ReplicatedMotions>().add_systems(Update,sync_frames.in_set(VoxelFrameSet::Replicate).after(ServerPlayerMovementSet::Apply).after(ServerPlayerRelocationSet::Sync));
        Self
    }
    pub fn run(&self)->Option<Vec<JoinHandle<()>>>{None}
}
fn sync_frames(trajectories:Option<Res<voxel_frame_kinematic_api::VoxelFrameTrajectories>>,mut replicated_motions:ResMut<ReplicatedMotions>,mut motions:MessageReader<server_voxel_frame_motion_api::SendVoxelFrameMotion>,time:Res<Time>,world:Res<ServerChunkWorld>,players:Res<ServerPlayerRegistry>,config:Res<ServerChunkResidencyConfig>,mut replicated:ResMut<ReplicatedFrames>,mut out:MessageWriter<ServerPacketOut>) {
    let online_players=players.players();
    let online:HashSet<_>=online_players.iter().map(|p|p.id).collect();
    replicated_motions.0.retain(|(player,_),_|online.contains(player));
    replicated.0.retain(|id,_|online.contains(id));
    let frames=world.frames();
    for player in online_players {
        let center=DVec3::new(player.position[0] as f64,player.position[1] as f64,player.position[2] as f64);
        let chunk=BlockPos::new(center.x.floor() as i32,center.y.floor() as i32,center.z.floor() as i32).chunk();
        let movement_epoch=players.movement_epoch(player.id).unwrap_or(0);
        let (epoch,sequence,known)=replicated.0.entry(player.id).or_default();
        let resnapshot=*epoch!=movement_epoch;
        *epoch=movement_epoch;
        let mut desired=HashMap::new();
        if let Some(root)=world.resident_key_for_player(player.id,chunk) {
            let radius=DVec3::splat((config.radius+1).max(1) as f64*16.0);
            for frame in frames.query(&root.scope(),VoxelBounds{min:(center-radius).to_array(),max:(center+radius).to_array()}) {
                desired.insert(frame.id,frame.clone());
                if let Some(previous)=known.get(&frame.id).filter(|previous|!resnapshot && previous.scope==frame.scope && previous.occupied_chunks==frame.occupied_chunks) {
                    if previous.transform!=frame.transform && trajectories.as_ref().and_then(|t|t.0.get(&(frame.scope.clone(),frame.id))).is_none() {
                        *sequence=sequence.checked_add(1).expect("frame stream sequence exhausted");
                        out.write(ServerPacketOut{audience:ServerAudience::Player(player.id),message:ClientBoundMessage::VoxelFramePose(VoxelFramePose{id:frame.id,affects_collision:true,trajectory:None,movement:FrameMovement::Instant,transform:frame.transform,revision:frame.revision,server_time_seconds:time.elapsed_secs_f64(),movement_epoch,stream_sequence:*sequence})});
                    }
                } else {
                    *sequence=sequence.checked_add(1).expect("frame stream sequence exhausted");
                        out.write(ServerPacketOut{audience:ServerAudience::Player(player.id),message:ClientBoundMessage::VoxelFrameUpsert(VoxelFrameUpsert{frame:frame.clone(),movement_epoch,stream_sequence:*sequence})});
                }
                if let Some(trajectory)=trajectories.as_ref().and_then(|t|t.0.get(&(frame.scope.clone(),frame.id))) {
                    if resnapshot || !known.contains_key(&frame.id) || known.get(&frame.id).is_some_and(|old|old.occupied_chunks!=frame.occupied_chunks) || replicated_motions.0.get(&(player.id,frame.id))!=Some(trajectory) {
                        replicated_motions.0.insert((player.id,frame.id),trajectory.clone());
                        *sequence+=1;
                        out.write(ServerPacketOut{audience:ServerAudience::Player(player.id),message:ClientBoundMessage::VoxelFramePose(VoxelFramePose{
                            id:frame.id,affects_collision:true,transform:trajectory.target,revision:frame.revision,movement:trajectory.movement.clone(),trajectory:Some(trajectory.clone()),server_time_seconds:time.elapsed_secs_f64(),movement_epoch,stream_sequence:*sequence
                        })});
                    }
                }
            }
        }
        for id in known.keys().filter(|id|!desired.contains_key(id)) {
            *sequence=sequence.checked_add(1).expect("frame stream sequence exhausted");
                        out.write(ServerPacketOut{audience:ServerAudience::Player(player.id),message:ClientBoundMessage::VoxelFrameRemove(VoxelFrameRemove{id:*id,movement_epoch,stream_sequence:*sequence})});
        }
        *known=desired;
    }
    for motion in motions.read() {
        if !motion.movement.valid(){warn!("invalid voxel-frame animation request");continue;}
        let Some((epoch,sequence,known))=replicated.0.get_mut(&motion.player_id) else {continue;};
        let Some(frame)=known.get(&motion.frame) else {continue;};
        *sequence=sequence.checked_add(1).expect("frame stream sequence exhausted");
        out.write(ServerPacketOut{audience:ServerAudience::Player(motion.player_id),message:ClientBoundMessage::VoxelFramePose(VoxelFramePose{
            id:motion.frame,affects_collision:false,trajectory:None,transform:motion.target,revision:frame.revision,movement:motion.movement.clone(),server_time_seconds:time.elapsed_secs_f64(),movement_epoch:*epoch,stream_sequence:*sequence
        })});
    }
}
