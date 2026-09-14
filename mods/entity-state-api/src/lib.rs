pub use generated_entity_registry::EntityKind;
pub use uuid::Uuid;
#[derive(Debug,Clone,PartialEq,serde::Serialize,serde::Deserialize)]
pub struct EntityPose { pub position:[f32;3], pub rotation:[f32;4], pub scale:f32 }
impl Default for EntityPose {
 fn default()->Self {Self{position:[0.0;3],rotation:[0.0,0.0,0.0,1.0],scale:1.0}}
}
impl EntityPose {
 pub fn valid(&self)->bool {
 let norm=self.rotation.iter().map(|x|x*x).sum::<f32>();
 self.position.iter().all(|x|x.is_finite()) && self.rotation.iter().all(|x|x.is_finite())
 && norm.is_finite() && norm>0.0001 && self.scale.is_finite() && self.scale>0.0
 }
}
#[derive(Debug,Clone,PartialEq,serde::Serialize,serde::Deserialize)]
pub struct EntityAnimationState { pub clip:String, pub speed:f32, pub repeat:bool, pub revision:u64 }
#[derive(Debug,Clone,PartialEq,serde::Serialize,serde::Deserialize)]
pub struct EntitySnapshot {pub uuid:Uuid,pub kind:EntityKind,pub pose:EntityPose,pub animation:Option<EntityAnimationState>}
