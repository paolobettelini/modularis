use entity_type_api::{EntityAnimation, EntityTypeInfo};
pub const ENTITY_INFO: EntityTypeInfo = EntityTypeInfo {
 id:"demo:jewel_beetle",
 model_path:Some("entity-jewel-beetle/models/jewel_beetle.blockymodel"),
 texture_path:Some("entity-jewel-beetle/textures/jewel_beetle.png"),
 texture_size:[512,512], primitive_scale:1.0/64.0, default_animation:Some("idle"),
 animations:&[
 EntityAnimation{id:"idle",path:"entity-jewel-beetle/animations/idle.blockyanim"},
 EntityAnimation{id:"walk",path:"entity-jewel-beetle/animations/walk.blockyanim"},
 ],
};
