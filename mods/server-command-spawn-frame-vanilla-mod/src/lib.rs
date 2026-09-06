use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_command_spawn_frame_lib::*;
use tokio::task::JoinHandle;
pub struct ServerCommandSpawnFrameVanillaMod;
impl ServerCommandSpawnFrameVanillaMod {
 pub fn init(bevy:&mut BevyMod,_commands:&mut impl server_command_api::ServerCommandApi,_chat:&mut impl server_chat_api::ServerChatApi,_world:&mut impl server_chunk_world_api::ServerChunkWorldApi,_stone:&mut block_stone::BlockStoneMod)->Self{
  let queue=SpawnFrameCommands::default();register(bevy.app.world().resource::<server_command_api::ServerCommandRegistry>(),&queue);
  bevy.app.insert_resource(queue).add_systems(Update,execute.in_set(server_chat_api::ServerChatSet::ApplyGameplay));Self
 }
 pub fn run(&self)->Option<Vec<JoinHandle<()>>>{None}
}
fn execute(queue:Res<SpawnFrameCommands>,world:Res<server_chunk_world_api::ServerChunkWorld>,mut chat:MessageWriter<server_chat_api::PublishServerChatMessage>){
 for message in apply(&queue,&world,generated_block_registry::BlockId::Stone.into()){chat.write(message);}
}
