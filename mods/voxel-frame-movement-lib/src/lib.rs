use serde::{Serialize,Deserialize};
use voxel_frame_api::VoxelFrameTransform;
#[derive(Debug,Clone,Copy,PartialEq,Default,Serialize,Deserialize)]
pub enum Easing {
    #[default] Linear,
    CubicBezier{x1:f32,y1:f32,x2:f32,y2:f32},
}
impl Easing {
    pub fn valid(self)->bool { match self {
        Self::Linear=>true,
        Self::CubicBezier{x1,y1,x2,y2}=>[x1,y1,x2,y2].iter().all(|v|v.is_finite()) && (0.0..=1.0).contains(&x1) && (0.0..=1.0).contains(&x2),
    }}
    pub fn sample(self,progress:f64)->f64 {
        let x=progress.clamp(0.0,1.0);
        if x==0.0 || x==1.0 {return x;}
        match self {
            Self::Linear=>x,
            Self::CubicBezier{x1,y1,x2,y2}=>{
                if !self.valid(){return x;}
                let curve=|t:f64,a:f32,b:f32|3.0*(1.0-t).powi(2)*t*a as f64+3.0*(1.0-t)*t*t*b as f64+t*t*t;
                // Solve Bezier X(t)=progress, then evaluate Y(t), as CSS does.
                let (mut lo,mut hi)=(0.0,1.0);
                for _ in 0..40 {let t=(lo+hi)*0.5;if curve(t,x1,x2)<x{lo=t}else{hi=t}}
                curve((lo+hi)*0.5,y1,y2)
            }
        }
    }
}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default,Serialize,Deserialize)]
pub enum RepeatMode { #[default] Once, Loop, PingPong }
#[derive(Debug,Clone,PartialEq,Default,Serialize,Deserialize)]
pub enum FrameMovement {
    #[default] Instant,
    Animated{duration_ms:u32,translation_easing:Easing,rotation_easing:Easing,repeat:RepeatMode},
}
impl FrameMovement {
    pub fn valid(&self)->bool {match self {Self::Instant=>true,Self::Animated{translation_easing,rotation_easing,..}=>translation_easing.valid()&&rotation_easing.valid()}}
    /// Returns a pose and whether a one-shot has finished. Zero duration is instant.
    pub fn sample(&self,start:VoxelFrameTransform,target:VoxelFrameTransform,elapsed_seconds:f64)->(VoxelFrameTransform,bool) {
        let Self::Animated{duration_ms,translation_easing,rotation_easing,repeat}=self else{return(target,true);};
        if *duration_ms==0 || !self.valid(){return(target,true);}
        let cycles=elapsed_seconds.max(0.0)*1000.0/(*duration_ms as f64);
        let progress=match repeat {RepeatMode::Once=>cycles.min(1.0),RepeatMode::Loop=>cycles.rem_euclid(1.0),RepeatMode::PingPong=>1.0-(cycles.rem_euclid(2.0)-1.0).abs()};
        let position=start.translation().lerp(target.translation(),translation_easing.sample(progress));
        let rotation=start.rotation().slerp(target.rotation(),rotation_easing.sample(progress));
        match VoxelFrameTransform::new(position.to_array(),rotation.to_array()) {
            Ok(pose)=>(pose,*repeat==RepeatMode::Once&&cycles>=1.0),
            Err(_)=>(target,true), // Extreme user-supplied easing must not panic the renderer.
        }
    }
}
#[cfg(test)] mod tests {
 use super::*;
 #[test] fn css_and_repeat_endpoints(){
  let a=VoxelFrameTransform::IDENTITY;let b=VoxelFrameTransform::new([10.0,0.0,0.0],[0.0,0.0,0.0,1.0]).unwrap();
  let motion=FrameMovement::Animated{duration_ms:1000,translation_easing:Easing::Linear,rotation_easing:Easing::Linear,repeat:RepeatMode::PingPong};
  assert_eq!(motion.sample(a,b,1.0).0.translation(),b.translation());
  assert_eq!(motion.sample(a,b,2.0).0.translation(),a.translation());
  assert!((Easing::CubicBezier{x1:0.42,y1:0.0,x2:0.58,y2:1.0}.sample(0.5)-0.5).abs()<1e-6);
 }
}

#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
pub struct FrameTrajectory {
 pub start:VoxelFrameTransform,pub target:VoxelFrameTransform,pub movement:FrameMovement,
 pub started_at_seconds:f64,
}
impl FrameTrajectory {
 pub fn sample(&self,now:f64)->(VoxelFrameTransform,bool){self.movement.sample(self.start,self.target,now-self.started_at_seconds)}
 /// Point velocity includes translation and rotation; point is frame-local.
 pub fn point_velocity(&self,local:[f64;3],now:f64)->[f64;3] {
  let (a,_)=self.sample(now-0.0005);let (b,_)=self.sample(now+0.0005);
  let mut p=a.translation();for axis in 0..3{p[axis]=local[axis];}
  ((b.local_to_world(p)-a.local_to_world(p))/0.001).to_array()
 }
}
