use bevy::prelude::*;
use collision_api::CharacterContact;
/// Latest resolved support; not tied to voxels or frame packet types.
#[derive(Resource,Default)]
pub struct PlayerSurfaceContact(pub Option<CharacterContact>);
#[derive(Resource,Default)]
pub struct PlayerSurfaceAttachment(pub Option<SurfaceAttachment>);
#[derive(Clone,Copy)]
pub struct SurfaceAttachment{
 pub surface:u128,pub local_foot:Vec3,pub translation:Vec3,pub rotation:Quat,
 pub velocity:Vec3,pub last_player_position:Vec3,
}
