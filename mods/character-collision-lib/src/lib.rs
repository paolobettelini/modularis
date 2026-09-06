//! Engine-independent kinematic capsule against oriented boxes.
use bevy::prelude::*;
use collision_api::*;
fn endpoints(q:CharacterQuery)->(Vec3,Vec3,f32){
 let r=q.radius.min(q.height*0.5);
 (q.position+q.up*r,q.position+q.up*(q.height-r),r)
}
fn bounds(q:CharacterQuery,extra:f32)->Aabb{
 let (a,b,r)=endpoints(q);let padding=Vec3::splat(r+extra);
 Aabb{min:a.min(b).min(a+q.displacement).min(b+q.displacement)-padding,
      max:a.max(b).max(a+q.displacement).max(b+q.displacement)+padding}
}
/// Exact closest point on a segment to an AABB: squared distance is piecewise quadratic.
fn segment_box(a:Vec3,b:Vec3,h:Vec3)->(Vec3,Vec3){
 let d=b-a;let mut breaks=vec![0.0,1.0];
 for axis in 0..3 {if d[axis].abs()>1e-12 {for side in [-h[axis],h[axis]]{
  let t=(side-a[axis])/d[axis];if t>0.0&&t<1.0{breaks.push(t);}
 }}}
 breaks.sort_by(f32::total_cmp);
 let mut best=(a,a.clamp(-h,h));let mut distance=f32::INFINITY;
 for interval in breaks.windows(2){
  let mid=(interval[0]+interval[1])*0.5;
  let (mut numerator,mut denominator)=(0.0,0.0);
  for axis in 0..3 {
   let v=a[axis]+d[axis]*mid;
   let side=if v< -h[axis]{-h[axis]}else if v>h[axis]{h[axis]}else{continue;};
   numerator+=d[axis]*(a[axis]-side);denominator+=d[axis]*d[axis];
  }
  let t=if denominator>0.0{(-numerator/denominator).clamp(interval[0],interval[1])}else{mid};
  let point=a+d*t;let on_box=point.clamp(-h,h);let sq=point.distance_squared(on_box);
  if sq<distance{distance=sq;best=(point,on_box);}
 }best
}
fn contact(q:CharacterQuery,b:CollisionBox)->(f32,CharacterContact){
 let (a,z,r)=endpoints(q);let inv=b.rotation.conjugate();
 let a=inv*(a-b.center);let z=inv*(z-b.center);
 let (p,c)=segment_box(a,z,b.half_extents);let delta=p-c;let length=delta.length();
 let (separation,normal,point)=if length>1e-8{(length-r,delta/length,c)}else{
  // Capsule axis intersects the box: use the smallest full separating translation.
  let mut depth=f32::INFINITY;let mut normal=Vec3::X;
  for axis in 0..3{
   let positive=b.half_extents[axis]-a[axis].min(z[axis])+r;
   let negative=b.half_extents[axis]+a[axis].max(z[axis])+r;
   if positive<depth{depth=positive;normal=Vec3::ZERO;normal[axis]=1.0;}
   if negative<depth{depth=negative;normal=Vec3::ZERO;normal[axis]=-1.0;}
  }(-depth,normal,c)
 };
 (separation,CharacterContact{normal:b.rotation*normal,point:b.center+b.rotation*point,surface:b.surface})
}
fn sweep(q:CharacterQuery,boxes:&[CollisionBox])->Option<(f32,CharacterContact)>{
 let length=q.displacement.length();if length<1e-10{return None;}
 let mut best=None;let mut best_time=1.0;
 for b in boxes{
  let mut t=0.0;
  for _ in 0..64{
   let (gap,hit)=contact(CharacterQuery{position:q.position+q.displacement*t,..q},*b);
   if gap<=q.skin()*1.1{
    if q.displacement.dot(hit.normal)< -1e-8 && t<=best_time{best_time=t;best=Some((t,hit));}break;
   }
   // Distance is 1-Lipschitz under translation, so this cannot skip a thin shape.
   t+=(gap-q.skin())/length;
   if t>best_time || t>1.0{break;}
  }
 }best
}
fn slide(mut q:CharacterQuery,boxes:&[CollisionBox])->CharacterResult{
 let mut contacts=Vec::new();
 for _ in 0..8{
  let deepest=boxes.iter().map(|b|contact(q,*b)).filter(|(d,_)|*d<0.0)
   .min_by(|a,b|a.0.total_cmp(&b.0));
  let Some((gap,hit))=deepest else{break;};
  q.position+=hit.normal*(-gap+q.skin());contacts.push(hit);
 }
 for _ in 0..8{
  let Some((t,hit))=sweep(q,boxes) else{q.position+=q.displacement;break;};
  q.position+=q.displacement*t;
  q.displacement*=1.0-t;
  contacts.push(hit);
  let slope=hit.normal.dot(q.up);
  if slope>0.0 && slope<q.slope_cosine {
   // Treat a non-walkable slope as a wall in the gravity plane. Removing
   // upward motion AFTER projecting onto the real surface points back into
   // that surface, causing repeated zero-time hits until the solver stalls.
   let wall=(hit.normal-q.up*slope).normalize_or_zero();
   contacts.push(CharacterContact{normal:wall,..hit});
  }
  // Clip against every accumulated plane, not independent global axis components.
  for _ in 0..3{for c in &contacts{let into=q.displacement.dot(c.normal);if into<0.0{q.displacement-=c.normal*into;}}}
  if q.displacement.length_squared()<q.skin()*q.skin(){break;}
 }
 CharacterResult{position:q.position,contacts,support:None}
}
pub fn support(q:CharacterQuery,geometry:&impl CharacterGeometry)->Option<CharacterContact>{
 let probe=CharacterQuery{displacement:-q.up*q.ground_probe,..q};
 let boxes=geometry.query(bounds(probe,q.skin()));
 sweep(probe,&boxes).map(|(_,hit)|hit).filter(|hit|hit.normal.dot(q.up)>=q.slope_cosine)
}
pub fn resolve(q:CharacterQuery,geometry:&impl CharacterGeometry)->CharacterResult{
 if !q.position.is_finite()||!q.displacement.is_finite()||q.up.length_squared()<0.9||q.radius<=0.0||q.height<=0.0{
  return CharacterResult{position:q.position,contacts:Vec::new(),support:None};
 }
 let mut area=bounds(q,q.skin());
 let raised=bounds(CharacterQuery{position:q.position+q.up*q.step_height,..q},q.skin());
 let lowered=bounds(CharacterQuery{position:q.position-q.up*q.ground_probe,..q},q.skin());
 area.min=area.min.min(raised.min).min(lowered.min);area.max=area.max.max(raised.max).max(lowered.max);
 let boxes=geometry.query(area).into_iter().filter(|b|q.ignore_surface!=Some(b.surface)).collect::<Vec<_>>();
 let mut result=slide(q,&boxes);
 let planar=q.displacement-q.up*q.displacement.dot(q.up);
 if q.was_grounded && q.step_height>0.0 && q.displacement.dot(q.up)<=q.skin()
    && (result.position-q.position).dot(planar)<planar.length_squared()-q.skin()*q.skin(){
  let raised=CharacterQuery{displacement:q.up*q.step_height,..q};
  if sweep(raised,&boxes).is_none(){
   let across=slide(CharacterQuery{position:q.position+q.up*q.step_height,displacement:planar,..q},&boxes);
   let down=CharacterQuery{position:across.position,displacement:-q.up*(q.step_height+q.ground_probe),..q};
   if let Some((t,hit))=sweep(down,&boxes){
    let landed=down.position+down.displacement*t;
    if hit.normal.dot(q.up)>=q.slope_cosine&&(landed-q.position).dot(planar)>(result.position-q.position).dot(planar)+q.skin()*q.skin(){
     result=CharacterResult{position:landed,contacts:across.contacts,support:Some(hit)};
    }
   }
  }
 }
 if q.was_grounded || q.displacement.dot(q.up)<=q.skin(){
  let probe=CharacterQuery{position:result.position,displacement:-q.up*q.ground_probe,..q};
  if let Some((t,hit))=sweep(probe,&boxes){
   if hit.normal.dot(q.up)>=q.slope_cosine{
    result.support=Some(hit);
    if q.was_grounded{result.position+=probe.displacement*t;}
   }
  }
 }
 if let Some(hit)=result.support{result.contacts.push(hit);}
 result
}
#[cfg(test)] mod tests{
 use super::*;
 struct Scene(Vec<CollisionBox>);impl CharacterGeometry for Scene{fn query(&self,_:Aabb)->Vec<CollisionBox>{self.0.clone()}}
 fn box_at(center:Vec3,half_extents:Vec3)->CollisionBox{CollisionBox{center,half_extents,rotation:Quat::IDENTITY,surface:0}}
 #[test] fn tilted_steep_contact_preserves_sideways_motion_with_rotated_gravity(){
  let gravity_rotation=Quat::from_rotation_x(0.8);
  let rotation=gravity_rotation*Quat::from_rotation_z(1.1);
  let up=gravity_rotation*Vec3::Y;
  let normal=rotation*Vec3::Y;
  let obstacle=CollisionBox{center:Vec3::ZERO,rotation,half_extents:Vec3::new(5.0,0.2,5.0),surface:9};
  let scene=Scene(vec![obstacle]);
  // Capsule bottom sphere is just inside the slope: recovery must also
  // handle overlaps smaller than skin, without losing tangent movement.
  let position=normal*(0.2+0.15-0.0001)-up*0.15;
  let sideways=gravity_rotation*Vec3::Z;
  let motion=sideways*0.2-up*0.1;
  let q=CharacterQuery::new(position,motion,up,0.15,0.9);
  let result=resolve(q,&scene);
  assert!((result.position-position).dot(sideways)>0.19);
  assert!(contact(CharacterQuery{position:result.position,..q},obstacle).0>=-q.skin());
 }
 #[test] fn fast_motion_does_not_skip_thin_model_elements(){
  let scene=Scene(vec![box_at(Vec3::new(1.0,1.0,0.0),Vec3::new(0.01,2.0,2.0))]);
  let result=resolve(CharacterQuery::new(Vec3::ZERO,Vec3::X*10.0,Vec3::Y,0.1,0.4),&scene);
  assert!(result.position.x<0.9);assert!(!result.contacts.is_empty());
 }
 #[test] fn scaled_character_passes_under_low_model(){
  let scene=Scene(vec![
   box_at(Vec3::new(0.0,-0.5,0.0),Vec3::new(5.0,0.5,2.0)),
   box_at(Vec3::new(1.0,1.0,0.0),Vec3::new(1.0,0.5,2.0))]);
  let small=resolve(CharacterQuery::new(Vec3::new(-1.0,0.001,0.0),Vec3::X*2.0,Vec3::Y,0.06,0.36),&scene);
  let large=resolve(CharacterQuery::new(Vec3::new(-1.0,0.001,0.0),Vec3::X*2.0,Vec3::Y,0.3,1.8),&scene);
  assert!(small.position.x>0.9);assert!(large.position.x<0.0);
 }
 #[test] fn supported_character_steps_up_without_global_axis_resolution(){
  for rot in [Quat::IDENTITY,Quat::from_rotation_x(1.1)]{
   let mut floor=box_at(Vec3::new(0.0,-0.5,0.0),Vec3::new(5.0,0.5,2.0));
   let mut step=box_at(Vec3::new(0.5,0.2,0.0),Vec3::new(0.5,0.2,2.0));
   for b in [&mut floor,&mut step]{b.center=rot*b.center;b.rotation=rot;}
   let scene=Scene(vec![floor,step]);
   let mut q=CharacterQuery::new(rot*Vec3::new(-0.7,0.001,0.0),rot*Vec3::X*1.3,rot*Vec3::Y,0.3,1.8);q.was_grounded=true;
   let result=resolve(q,&scene);
   assert!(result.position.dot(rot*Vec3::X)>0.4);assert!(result.position.dot(rot*Vec3::Y)>0.39);
  }
 }
 #[test] fn steep_surface_is_not_walkable(){
  let rotation=Quat::from_rotation_z(1.1);
  let scene=Scene(vec![CollisionBox{center:Vec3::ZERO,rotation,half_extents:Vec3::new(4.0,0.2,4.0),surface:7}]);
  let result=resolve(CharacterQuery::new(Vec3::Y*3.0,Vec3::NEG_Y*4.0,Vec3::Y,0.15,0.9),&scene);
  assert!(result.support.is_none());
 }
 #[test] fn upward_jump_does_not_snap_back_to_floor(){
  let scene=Scene(vec![box_at(Vec3::new(0.0,-0.5,0.0),Vec3::new(5.0,0.5,5.0))]);
  let result=resolve(CharacterQuery::new(Vec3::Y*0.001,Vec3::Y*0.2,Vec3::Y,0.3,1.8),&scene);
  assert!(result.support.is_none());assert!(result.position.y>0.19);
 }
 #[test] fn gravity_rotation_preserves_floor_contact(){
  for rotation in [Quat::IDENTITY,Quat::from_rotation_z(1.2),Quat::from_rotation_x(2.0)]{
   let scene=Scene(vec![CollisionBox{center:rotation*Vec3::new(0.0,-0.5,0.0),rotation,half_extents:Vec3::new(5.0,0.5,5.0),surface:0}]);
   let q=CharacterQuery::new(rotation*Vec3::Y,rotation*Vec3::new(0.2,-2.0,0.0),rotation*Vec3::Y,0.3,1.8);
   let result=resolve(q,&scene);assert!(result.support.is_some());assert!((result.position.dot(rotation*Vec3::Y)).abs()<0.005);
  }
 }
}
