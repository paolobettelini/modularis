use bevy::prelude::*;
use std::sync::{Arc,Mutex};
use server_command_api::{ServerCommandRegistry,ServerCommandSource,brigadier::{builder::{argument_builder::ArgumentBuilder,literal_argument_builder::literal,required_argument_builder::Argument},arguments::string_argument_type::{get_string,greedy_string},context::CommandContext}};
use server_chat_api::PublishServerChatMessage;
#[derive(Resource,Default,Clone)]
pub struct SpawnFrameCommands(Arc<Mutex<Vec<(u64,String)>>>);
pub fn register(registry:&ServerCommandRegistry,queue:&SpawnFrameCommands){
 let queue=queue.0.clone();
 let arguments=ArgumentBuilder::new(Argument::<ServerCommandSource>::new("coordinates",Arc::new(greedy_string()),None).into())
  .executes(move |context:&CommandContext<ServerCommandSource>|{
   queue.lock().unwrap().push((context.source.player_id,get_string(context,"coordinates").unwrap_or_default()));1
  });
 registry.register_restricted("spawnframe",generated_permission_registry::PermissionId::Privileged,literal("spawnframe").then(arguments));
}
/// Call under custom scope/phase policy; block choice belongs to the caller.
pub fn apply(queue:&SpawnFrameCommands,world:&server_chunk_world_api::ServerChunkWorld,block:block_state_api::BlockState)->Vec<PublishServerChatMessage>{
 std::mem::take(&mut *queue.0.lock().unwrap()).into_iter().map(|(player,args)|{
  let result=(||{
   let values=args.split_whitespace().map(str::parse::<f64>).collect::<Result<Vec<_>,_>>().map_err(|_|"Usage: /spawnframe x y z i j k (rotation in degrees)".to_string())?;
   let v:[f64;6]=values.try_into().map_err(|_|"Expected six numbers".to_string())?;
   server_voxel_frame_spawn_lib::spawn_frame(world,server_chunk_provider_api::ChunkViewer::Player(player),[v[0],v[1],v[2]],[v[3],v[4],v[5]],block.clone()).map(|id|format!("Spawned frame {id}"))
  })();
  PublishServerChatMessage{audience:audience_api::Audience::personal(player),text:result.unwrap_or_else(|e|e)}
 }).collect()
}
