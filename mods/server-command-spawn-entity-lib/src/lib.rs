use bevy::prelude::*;
use std::sync::{Arc,Mutex};
use server_command_api::{ServerCommandRegistry,ServerCommandSource,brigadier::{
 builder::{argument_builder::ArgumentBuilder,literal_argument_builder::literal,required_argument_builder::Argument},
 arguments::string_argument_type::{get_string,greedy_string},context::CommandContext,
 suggestion::{SuggestionProvider,Suggestions,SuggestionsBuilder}}};
use entity_state_api::*;
use server_entity_api::EntityRequest;
#[derive(Resource,Default,Clone)]
pub struct SpawnEntityCommands(Arc<Mutex<Vec<(u64,String)>>>);
impl SpawnEntityCommands {
 pub fn drain(&self)->Vec<(u64,String)>{std::mem::take(&mut *self.0.lock().unwrap())}
}
struct EntitySuggestions;
impl SuggestionProvider<ServerCommandSource> for EntitySuggestions{
 fn get_suggestions(&self,_:CommandContext<ServerCommandSource>,builder:SuggestionsBuilder)->Suggestions{
 let raw=builder.remaining().to_string();
 let mut offset=0;
 for _ in 0..3 {
 let rest=&raw[offset..];
 let trimmed=rest.trim_start();offset+=rest.len()-trimmed.len();
 let Some(end)=trimmed.find(char::is_whitespace)else{return builder.build();};
 offset+=end;
 }
 let rest=&raw[offset..];offset+=rest.len()-rest.trim_start().len();
 let prefix=&raw[offset..];
 if prefix.chars().any(char::is_whitespace){return builder.build();}
 let mut b=builder.create_offset(builder.start()+offset);
 for kind in generated_entity_registry::all_entities(){
 let id=generated_entity_registry::id(*kind);
 if id.starts_with(prefix){b=b.suggest(id);}
 }
 b.build()
 }
}
pub fn register(registry:&ServerCommandRegistry,queue:&SpawnEntityCommands){
 let queue=queue.0.clone();
 let args=ArgumentBuilder::new(Argument::<ServerCommandSource>::new("arguments",Arc::new(greedy_string()),Some(Arc::new(EntitySuggestions))).into())
 .executes(move |ctx:&CommandContext<ServerCommandSource>|{queue.lock().unwrap().push((ctx.source.player_id,get_string(ctx,"arguments").unwrap_or_default()));1});
 registry.register_restricted("spawnentity",generated_permission_registry::PermissionId::Privileged,literal("spawnentity").then(args));
}
/// Reusable creation; caller supplies world and chooses whether to apply it.
pub fn spawn_request(args:&str,world:world_instance_api::WorldScopeId)->Result<EntityRequest,String>{
 let a:Vec<_>=args.split_whitespace().collect();
 if !(4..=5).contains(&a.len()){return Err("Usage: /spawnentity x y z <id> [scale]".into());}
 let position=[a[0],a[1],a[2]].map(str::parse::<f32>);
 let position=[position[0].clone().map_err(|_|"Invalid x")?,position[1].clone().map_err(|_|"Invalid y")?,position[2].clone().map_err(|_|"Invalid z")?];
 let kind=if let Some(kind)=generated_entity_registry::from_str(a[3]){kind}else{
 let mut matches=generated_entity_registry::all_entities().iter().copied().filter(|kind|generated_entity_registry::id(*kind).split(':').next_back()==Some(a[3]));
 let kind=matches.next().ok_or("Unknown entity type")?;
 if matches.next().is_some(){return Err("Ambiguous entity ID; use its namespace".into());}kind
 };
 let scale=a.get(4).map(|s|s.parse::<f32>()).transpose().map_err(|_|"Invalid scale")?.unwrap_or(1.0);
 let pose=EntityPose{position,scale,..Default::default()};
 if !pose.valid(){return Err("Coordinates must be finite and scale must be positive".into());}
 let info=generated_entity_registry::info(kind);
 let animation=info.default_animation.and_then(|id|info.animations.iter().find(|a|a.id==id)).map(|a|EntityAnimationState{clip:a.id.into(),speed:1.0,repeat:true,revision:0});
 Ok(EntityRequest::Spawn{entity:EntitySnapshot{uuid:Uuid::new_v4(),kind,pose,animation},world})
}
