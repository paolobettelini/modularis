use bevy::prelude::*;
use server_command_spawn_entity_lib::*;
use server_entity_api::*;
pub struct ServerCommandSpawnEntityVanillaMod;
impl ServerCommandSpawnEntityVanillaMod{
 pub fn init(bevy:&mut bevy_mod::BevyMod,_commands:&mut impl server_command_api::ServerCommandApi,_chat:&mut impl server_chat_api::ServerChatApi,_worlds:&mut impl server_chunk_world_api::ServerChunkWorldApi,_entities:&mut server_entity_state_mod::ServerEntityStateMod)->Self{
 let queue=SpawnEntityCommands::default();register(bevy.app.world().resource::<server_command_api::ServerCommandRegistry>(),&queue);
 bevy.app.insert_resource(queue).add_systems(Update,execute.in_set(EntitySetAdapter::Command).after(server_chat_api::ServerChatSet::ApplyGameplay).before(ServerEntitySet::Apply));Self
 } pub fn run(&self)->Option<Vec<tokio::task::JoinHandle<()>>>{None}
}
#[derive(SystemSet,Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum EntitySetAdapter{Command}
fn execute(queue:Res<SpawnEntityCommands>,worlds:Res<server_chunk_world_api::ServerChunkWorld>,mut requests:MessageWriter<EntityRequest>,mut chat:MessageWriter<server_chat_api::PublishServerChatMessage>){
 for (player,args) in queue.drain(){
 let result=worlds.resident_key_for_player(player,voxel_math_api::ChunkPos::new(0,0,0)).ok_or("Player has no world route".to_string()).and_then(|key|spawn_request(&args,key.scope()));
 let text=match result{Ok(request)=>{let id=match &request{EntityRequest::Spawn{entity,..}=>entity.uuid.to_string(),_=>unreachable!()};requests.write(request);format!("Spawned entity {id}")},Err(error)=>error};
 chat.write(server_chat_api::PublishServerChatMessage{audience:audience_api::Audience::personal(player),text});
 }
}
