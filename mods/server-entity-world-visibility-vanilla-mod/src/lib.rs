use bevy::prelude::*;
use server_entity_api::*;
use server_entity_visibility_api::*;
pub struct ServerEntityWorldVisibilityVanillaMod;
impl ServerEntityWorldVisibilityVanillaMod{
 pub fn init(bevy:&mut bevy_mod::BevyMod,_entities:&mut server_entity_state_mod::ServerEntityStateMod,_worlds:&mut impl server_chunk_world_api::ServerChunkWorldApi,_players:&mut impl server_player_registry_api::ServerPlayerRegistryApi)->Self{
 bevy.app.init_resource::<EntityViewers>().add_systems(Update,select.in_set(ServerEntitySet::Visibility).after(server_player_registry_api::ServerPlayerRelocationSet::Apply));Self
 } pub fn run(&self)->Option<Vec<tokio::task::JoinHandle<()>>>{None}
}
fn select(entities:Query<(&ServerEntity,&EntityWorld)>,players:Res<server_player_registry_api::ServerPlayerRegistry>,worlds:Res<server_chunk_world_api::ServerChunkWorld>,mut viewers:ResMut<EntityViewers>){
 viewers.0.clear();
 for (entity,scope) in &entities {
 viewers.0.insert(entity.0.uuid,players.players().into_iter().filter(|p|worlds.resident_key_for_player(p.id,voxel_math_api::BlockPos::new(p.position[0].floor() as i32,p.position[1].floor() as i32,p.position[2].floor() as i32).chunk()).is_some_and(|key|key.scope()==scope.0)).map(|p|p.id).collect());
 }
}
