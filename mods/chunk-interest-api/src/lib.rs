//! Pure shared interest mechanics; consumers select their volume policy.
use std::sync::Arc;
use voxel_math_api::ChunkPos;

pub trait ChunkInterestVolume: Send + Sync + 'static {
    fn contains_offset(&self, offset: [i64; 3], radius: i32) -> bool;
    fn offsets(&self, radius: i32) -> Vec<[i32; 3]> {
        let r = radius.max(0);
        let mut result = Vec::new();
        for z in -r..=r { for y in -r..=r { for x in -r..=r {
            if self.contains_offset([x as i64,y as i64,z as i64],r) { result.push([x,y,z]); }
        }}}
        result
    }
}
#[derive(Clone)]
pub struct ChunkInterest(pub Arc<dyn ChunkInterestVolume>);
impl ChunkInterest {
    pub fn new(volume: impl ChunkInterestVolume) -> Self { Self(Arc::new(volume)) }
    pub fn contains(&self, center: ChunkPos, p: ChunkPos, radius: i32) -> bool {
        self.0.contains_offset([p.x as i64-center.x as i64,p.y as i64-center.y as i64,p.z as i64-center.z as i64],radius)
    }
    pub fn positions(&self, center: ChunkPos, radius: i32) -> Vec<ChunkPos> {
        self.0.offsets(radius).into_iter().filter_map(|d| Some(ChunkPos::new(center.x.checked_add(d[0])?,center.y.checked_add(d[1])?,center.z.checked_add(d[2])?))).collect()
    }
}
/// An opt-in isotropic mechanism. World/chunk storage never selects it.
#[derive(Debug, Clone, Copy)]
pub struct SphericalChunkInterest;
impl ChunkInterestVolume for SphericalChunkInterest {
    fn contains_offset(&self, d: [i64; 3], radius: i32) -> bool {
        let r = i128::from(radius.max(0));
        d.into_iter().map(|v| i128::from(v).pow(2)).sum::<i128>() <= r*r
    }
}
pub fn distance_squared(a: ChunkPos,b: ChunkPos) -> u64 {
    [a.x as i64-b.x as i64,a.y as i64-b.y as i64,a.z as i64-b.z as i64].into_iter()
        .fold(0u64,|n,d| n.saturating_add(d.unsigned_abs().saturating_mul(d.unsigned_abs())))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn isotropic_and_translation_invariant() {
        let volume=ChunkInterest::new(SphericalChunkInterest);
        let center=ChunkPos::new(-20,30,-40);
        let points=volume.positions(center,4);
        assert!(volume.contains(center,ChunkPos::new(-20,34,-40),4));
        for p in points {
            let d=[p.x-center.x,p.y-center.y,p.z-center.z];
            for q in [[d[1],d[2],d[0]],[-d[0],-d[1],-d[2]]] {
                assert!(volume.contains(center,ChunkPos::new(center.x+q[0],center.y+q[1],center.z+q[2]),4));
            }
        }
        assert_eq!(volume.positions(ChunkPos::new(1,0,0),4).len(),volume.positions(ChunkPos::new(0,1,0),4).len());
    }
}
