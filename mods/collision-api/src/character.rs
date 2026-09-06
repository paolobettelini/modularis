use bevy::prelude::*;

/// Kinematic capsule. Position is the foot point; axis is normalized player up.
#[derive(Debug,Clone,Copy)]
pub struct CharacterQuery {
    pub position:Vec3, pub displacement:Vec3, pub up:Vec3,
    pub radius:f32, pub height:f32,
    pub step_height:f32, pub ground_probe:f32, pub slope_cosine:f32,
    pub was_grounded:bool,
    /// Used while transporting an attachment: exclude its own moving surface.
    pub ignore_surface:Option<u128>,
}
impl CharacterQuery {
    pub fn new(position:Vec3,displacement:Vec3,up:Vec3,radius:f32,height:f32)->Self{
        let scale=(height/1.8).max(0.0001);
        Self{position,displacement,up:up.normalize_or_zero(),radius,height,
            step_height:0.55*scale,ground_probe:0.025*scale,slope_cosine:0.70710677,was_grounded:false,ignore_surface:None}
    }
    pub fn skin(self)->f32{(self.radius.min(self.height)*0.002).max(1e-7)}
}
#[derive(Debug,Clone,Copy)]
pub struct CollisionBox {
    pub center:Vec3, pub rotation:Quat, pub half_extents:Vec3,
    /// Opaque moving surface identity. Zero denotes the static root.
    pub surface:u128,
}
#[derive(Debug,Clone,Copy)]
pub struct CharacterContact {pub normal:Vec3,pub point:Vec3,pub surface:u128}
#[derive(Debug,Clone)]
pub struct CharacterResult {pub position:Vec3,pub contacts:Vec<CharacterContact>,pub support:Option<CharacterContact>}
impl CharacterResult {
    pub fn project_velocity(&self,mut velocity:Vec3)->Vec3{
        for _ in 0..3 {for contact in &self.contacts {
            let into=velocity.dot(contact.normal);if into<0.0{velocity-=contact.normal*into;}
        }} velocity
    }
}
/// Geometry query only: no voxel/frame identities or gameplay rules in the solver.
pub trait CharacterGeometry:Send+Sync {
    fn query(&self,bounds:super::Aabb)->Vec<CollisionBox>;
}
pub trait CharacterCollisionBackend:Send+Sync+'static {
    fn resolve(&self,query:CharacterQuery)->CharacterResult;
    fn support(&self,query:CharacterQuery)->Option<CharacterContact>;
    fn surface_pose(&self,_surface:u128)->Option<(Vec3,Quat)>{None}
}

#[derive(SystemSet,Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum CharacterCollisionSet {Rebase,Resolve}
