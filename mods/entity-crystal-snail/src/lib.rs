use entity_type_api::{EntityAnimation, EntityTypeInfo};
pub const ENTITY_INFO: EntityTypeInfo = EntityTypeInfo {
 id:"demo:crystal_snail",
 model_path:Some("entity-crystal-snail/models/crystal_snail.blockymodel"),
 texture_path:Some("entity-crystal-snail/textures/crystal_snail.png"),
 texture_size:[512,512], primitive_scale:1.0/64.0, default_animation:Some("idle"),
 animations:&[
 EntityAnimation{id:"idle",path:"entity-crystal-snail/animations/idle.blockyanim"},
 EntityAnimation{id:"walk",path:"entity-crystal-snail/animations/walk.blockyanim"},
 ],
};
