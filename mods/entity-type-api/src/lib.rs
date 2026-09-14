pub struct EntityAnimation { pub id: &'static str, pub path: &'static str }
pub struct EntityTypeInfo {
 pub id: &'static str, pub model_path: Option<&'static str>,
 pub texture_path: Option<&'static str>, pub texture_size:[u32;2],
 pub primitive_scale:f32, pub animations:&'static [EntityAnimation],
 pub default_animation:Option<&'static str>,
}
