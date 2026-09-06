use bevy::{math::DVec3, prelude::*};
use std::{collections::{HashMap, HashSet}, sync::{Arc, RwLock}};
use voxel_frame_api::*;
use voxel_math_api::ChunkPos;
use world_instance_api::WorldScopeId;

/// Broad phase only. Exact ray/shape tests remain in the geometry consumer.
pub trait VoxelFrameSpatialIndex: Send + Sync + 'static {
    fn update(&mut self, id: VoxelFrameId, bounds: Option<VoxelBounds>);
    fn remove(&mut self, id: VoxelFrameId);
    fn query(&self, bounds: VoxelBounds) -> Vec<VoxelFrameId>;
}

/// Median-split BVH broad phase. Bounds changes rebuild the small metadata
/// tree, not voxel meshes; queries prune all three axes. A dynamic tree or
/// spatial hash can implement the same interface for frequently moving frames.
#[derive(Default)]
pub struct BvhFrameIndex {
    entries: HashMap<VoxelFrameId,VoxelBounds>,
    root: Option<Box<BvhNode>>,
}
enum BvhNode {
    Leaf(VoxelFrameId,VoxelBounds),
    Branch { bounds: VoxelBounds, left: Box<BvhNode>, right: Box<BvhNode> },
}
impl BvhNode {
    fn bounds(&self)->VoxelBounds { match self {Self::Leaf(_,b)=>*b,Self::Branch{bounds,..}=>*bounds} }
    fn build(mut entries:Vec<(VoxelFrameId,VoxelBounds)>)->Option<Box<Self>> {
        if entries.is_empty(){return None;}
        if entries.len()==1 {let (id,b)=entries[0];return Some(Box::new(Self::Leaf(id,b)));}
        let bounds=entries.iter().map(|(_,b)|*b).reduce(VoxelBounds::union).unwrap();
        let axis=(0..3).max_by(|a,b|(bounds.max[*a]-bounds.min[*a]).total_cmp(&(bounds.max[*b]-bounds.min[*b]))).unwrap();
        entries.sort_unstable_by(|a,b|(a.1.min[axis]+a.1.max[axis]).total_cmp(&(b.1.min[axis]+b.1.max[axis])));
        let right=entries.split_off(entries.len()/2);
        Some(Box::new(Self::Branch{bounds,left:Self::build(entries).unwrap(),right:Self::build(right).unwrap()}))
    }
    fn query(&self,bounds:VoxelBounds,result:&mut Vec<VoxelFrameId>) {
        if !self.bounds().overlaps(bounds){return;}
        match self {Self::Leaf(id,_)=>result.push(*id),Self::Branch{left,right,..}=>{left.query(bounds,result);right.query(bounds,result);}}
    }
}
impl BvhFrameIndex {
    fn rebuild(&mut self){self.root=BvhNode::build(self.entries.iter().map(|(id,b)|(*id,*b)).collect());}
}
impl VoxelFrameSpatialIndex for BvhFrameIndex {
    fn update(&mut self,id:VoxelFrameId,bounds:Option<VoxelBounds>){
        if self.entries.get(&id).copied()==bounds{return;}
        if let Some(bounds)=bounds{self.entries.insert(id,bounds);}else{self.entries.remove(&id);}
        self.rebuild();
    }
    fn remove(&mut self,id:VoxelFrameId){if self.entries.remove(&id).is_some(){self.rebuild();}}
    fn query(&self,bounds:VoxelBounds)->Vec<VoxelFrameId>{let mut result=Vec::new();if let Some(root)=&self.root{root.query(bounds,&mut result);}result}
}

#[derive(Message, Debug, Clone)]
pub enum VoxelFrameChanged {
    Upsert(VoxelFrame),
    Removed { scope: WorldScopeId, id: VoxelFrameId },
}

struct ScopedFrames {
    frames: HashMap<VoxelFrameId, VoxelFrame>,
    index: Box<dyn VoxelFrameSpatialIndex>,
}
struct RegistryState {
    scopes: HashMap<WorldScopeId, ScopedFrames>,
    changes: Vec<VoxelFrameChanged>,
    revision: u64,
}
type IndexFactory = Arc<dyn Fn() -> Box<dyn VoxelFrameSpatialIndex> + Send + Sync>;

/// Structural metadata only: no chunk payloads, generator, damage or policy.
/// IDs are unique within a WorldScope. The implicit root is not an allocation.
#[derive(Resource, Clone)]
pub struct VoxelFrames {
    state: Arc<RwLock<RegistryState>>,
    index_factory: IndexFactory,
}
impl Default for VoxelFrames {
    fn default() -> Self { Self::new(|| Box::<BvhFrameIndex>::default()) }
}
impl VoxelFrames {
    pub fn new(factory: impl Fn() -> Box<dyn VoxelFrameSpatialIndex> + Send + Sync + 'static) -> Self {
        Self { state: Arc::new(RwLock::new(RegistryState { scopes: HashMap::new(), changes: Vec::new(), revision: 0 })), index_factory: Arc::new(factory) }
    }
    pub fn revision(&self) -> u64 { self.state.read().unwrap().revision }
    pub fn get(&self, scope: &WorldScopeId, id: VoxelFrameId) -> Option<VoxelFrame> {
        self.state.read().unwrap().scopes.get(scope)?.frames.get(&id).cloned()
    }
    pub fn transform(&self, scope: &WorldScopeId, id: VoxelFrameId) -> Option<VoxelFrameTransform> {
        if id.is_root() { Some(VoxelFrameTransform::IDENTITY) } else { self.get(scope,id).map(|f| f.transform) }
    }
    pub fn upsert(&self, frame: VoxelFrame) -> Result<(), &'static str> {
        if frame.id.is_root() { return Err("the root frame is implicit and cannot be moved"); }
        let mut state = self.state.write().unwrap();
        let scoped = state.scopes.entry(frame.scope.clone()).or_insert_with(|| ScopedFrames {
            frames: HashMap::new(), index: (self.index_factory)(),
        });
        if scoped.frames.get(&frame.id) == Some(&frame) { return Ok(()); }
        scoped.index.update(frame.id, frame.world_bounds());
        scoped.frames.insert(frame.id,frame.clone());
        state.revision = state.revision.wrapping_add(1);
        state.changes.push(VoxelFrameChanged::Upsert(frame));
        Ok(())
    }
    pub fn set_transform(&self, scope: &WorldScopeId, id: VoxelFrameId, transform: VoxelFrameTransform) -> Result<(), &'static str> {
        self.modify(scope,id,|frame| {
            if frame.transform==transform {return false;}
            frame.transform=transform;true
        })
    }
    fn modify(&self,scope:&WorldScopeId,id:VoxelFrameId,change:impl FnOnce(&mut VoxelFrame)->bool)->Result<(), &'static str> {
        let mut state=self.state.write().unwrap();
        let scoped=state.scopes.get_mut(scope).ok_or("unknown frame scope")?;
        let frame=scoped.frames.get_mut(&id).ok_or("unknown frame")?;
        if !change(frame){return Ok(());}
        frame.revision=frame.revision.wrapping_add(1);
        let updated=frame.clone();
        scoped.index.update(id,updated.world_bounds());
        state.revision=state.revision.wrapping_add(1);
        state.changes.push(VoxelFrameChanged::Upsert(updated));
        Ok(())
    }
    pub fn set_occupied(&self, scope: &WorldScopeId, id: VoxelFrameId, position: ChunkPos, occupied: bool) {
        let _=self.modify(scope,id,|frame| {
            if occupied {frame.occupied_chunks.insert(position)} else {frame.occupied_chunks.remove(&position)}
        });
    }
    pub fn remove(&self, scope: &WorldScopeId, id: VoxelFrameId) {
        let mut state = self.state.write().unwrap();
        if let Some(scoped) = state.scopes.get_mut(scope) {
            if scoped.frames.remove(&id).is_none() { return; }
            scoped.index.remove(id);
            state.revision = state.revision.wrapping_add(1);
            state.changes.push(VoxelFrameChanged::Removed { scope: scope.clone(), id });
        }
    }
    pub fn in_scope(&self, scope: &WorldScopeId) -> Vec<VoxelFrame> {
        self.state.read().unwrap().scopes.get(scope).map(|s| s.frames.values().cloned().collect()).unwrap_or_default()
    }
    pub fn scopes(&self)->Vec<WorldScopeId>{self.state.read().unwrap().scopes.keys().cloned().collect()}
    pub fn all(&self) -> Vec<VoxelFrame> {
        self.state.read().unwrap().scopes.values().flat_map(|s| s.frames.values().cloned()).collect()
    }
    pub fn query(&self, scope: &WorldScopeId, bounds: VoxelBounds) -> Vec<VoxelFrame> {
        let state = self.state.read().unwrap();
        let Some(scoped) = state.scopes.get(scope) else { return Vec::new(); };
        scoped.index.query(bounds).iter().filter_map(|id| scoped.frames.get(id).cloned()).collect()
    }
    /// Frame-local sparse chunks only; transform changes do not change addresses.
    pub fn interested_chunks(&self, scope: &WorldScopeId, center: DVec3, radius: f64) -> HashSet<VoxelChunkAddress> {
        let r = DVec3::splat(radius);
        let bounds = VoxelBounds { min: (center-r).to_array(), max: (center+r).to_array() };
        let mut result = HashSet::new();
        for frame in self.query(scope,bounds) {
            for position in &frame.occupied_chunks {
                let chunk_bounds = frame.transform.transform_bounds(VoxelBounds::chunk(*position));
                let closest = center.clamp(DVec3::from_array(chunk_bounds.min),DVec3::from_array(chunk_bounds.max));
                if closest.distance_squared(center) <= radius * radius {
                    result.insert(VoxelChunkAddress::new(frame.id,*position));
                }
            }
        }
        result
    }
    /// Drain once in the ECS adapter, then fan out through a Message.
    pub fn drain_changes(&self) -> Vec<VoxelFrameChanged> { std::mem::take(&mut self.state.write().unwrap().changes) }
}

#[derive(SystemSet,Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum VoxelFrameSet { CollectChanges, Replicate, Persist }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn spatial_index_tracks_pose_changes_without_changing_local_occupancy() {
        let frames=VoxelFrames::default();
        let scope=WorldScopeId::new(world_instance_api::WorldInstanceId::new("test:world"),"test:source");
        let id=VoxelFrameId::new();
        frames.upsert(VoxelFrame::new(id,scope.clone(),VoxelFrameTransform::IDENTITY)).unwrap();
        let local=ChunkPos::new(-1,2,3);
        frames.set_occupied(&scope,id,local,true);
        let bounds=VoxelBounds::chunk(local);
        assert_eq!(frames.query(&scope,bounds).len(),1);
        frames.set_transform(&scope,id,VoxelFrameTransform::new([1000.0,0.0,0.0],[0.0,0.0,0.0,1.0]).unwrap()).unwrap();
        assert!(frames.query(&scope,bounds).is_empty());
        assert!(frames.get(&scope,id).unwrap().occupied_chunks.contains(&local));
        frames.remove(&scope,id);
        assert!(frames.in_scope(&scope).is_empty());
    }
}
