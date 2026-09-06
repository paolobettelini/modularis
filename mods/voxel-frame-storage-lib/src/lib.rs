//! Independent versioned frame catalogue. Chunk and component codecs are reused unchanged.
use voxel_frame_api::*;
use voxel_math_api::ChunkPos;
use world_instance_api::{WorldInstanceId,WorldScopeId};

pub fn encode(frames:&[VoxelFrame])->Result<Vec<u8>,String> {
    let mut out=b"PWVF".to_vec();
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&u32::try_from(frames.len()).map_err(|_|"too many frames")?.to_le_bytes());
    for frame in frames {
        if frame.id.is_root() { return Err("root is implicit".into()); }
        out.extend_from_slice(frame.id.0.as_bytes());
        for text in [frame.scope.instance.as_str(),frame.scope.source.as_str()] {
            out.extend_from_slice(&u32::try_from(text.len()).map_err(|_|"scope too long")?.to_le_bytes());
            out.extend_from_slice(text.as_bytes());
        }
        for value in frame.transform.translation().to_array().into_iter().chain(frame.transform.rotation().to_array()) { out.extend_from_slice(&value.to_le_bytes()); }
        out.extend_from_slice(&frame.revision.to_le_bytes());
        out.extend_from_slice(&u32::try_from(frame.occupied_chunks.len()).map_err(|_|"too many chunks")?.to_le_bytes());
        for p in &frame.occupied_chunks { for v in [p.x,p.y,p.z] { out.extend_from_slice(&v.to_le_bytes()); } }
    }
    Ok(out)
}
pub fn decode(bytes:&[u8])->Result<Vec<VoxelFrame>,String> {
    let mut c=Cursor(bytes);
    if c.take(4)?!=b"PWVF" || u16::from_le_bytes(c.take(2)?.try_into().unwrap())!=1 { return Err("unsupported voxel frame catalogue".into()); }
    let count=c.u32()? as usize;
    if count>bytes.len()/84 { return Err("invalid frame count".into()); }
    let mut frames=Vec::new();
    let mut ids=std::collections::HashSet::new();
    for _ in 0..count {
        let id=VoxelFrameId(uuid::Uuid::from_slice(c.take(16)?).map_err(|e|e.to_string())?);
        let scope=WorldScopeId::new(WorldInstanceId::new(c.text()?),c.text()?);
        if id.is_root() || !ids.insert((scope.clone(),id)) { return Err("duplicate or root frame in catalogue".into()); }
        let mut t=[0.0;3]; let mut q=[0.0;4];
        for v in t.iter_mut().chain(q.iter_mut()) { *v=f64::from_le_bytes(c.take(8)?.try_into().unwrap()); }
        let mut frame=VoxelFrame::new(id,scope,VoxelFrameTransform::new(t,q)?);
        frame.revision=u64::from_le_bytes(c.take(8)?.try_into().unwrap());
        let chunks=c.u32()? as usize;
        if chunks>c.0.len()/12 { return Err("invalid occupied chunk count".into()); }
        for _ in 0..chunks { frame.occupied_chunks.insert(ChunkPos::new(c.i32()?,c.i32()?,c.i32()?)); }
        frames.push(frame);
    }
    if !c.0.is_empty() { return Err("trailing frame catalogue data".into()); }
    Ok(frames)
}
struct Cursor<'a>(&'a[u8]);
impl<'a> Cursor<'a> {
    fn take(&mut self,n:usize)->Result<&'a[u8],String>{if n>self.0.len(){return Err("truncated frame catalogue".into());} let (a,b)=self.0.split_at(n);self.0=b;Ok(a)}
    fn u32(&mut self)->Result<u32,String>{Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))}
    fn i32(&mut self)->Result<i32,String>{Ok(i32::from_le_bytes(self.take(4)?.try_into().unwrap()))}
    fn text(&mut self)->Result<String,String>{let n=self.u32()? as usize;String::from_utf8(self.take(n)?.to_vec()).map_err(|e|e.to_string())}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn catalogue_roundtrip_keeps_pose_and_negative_local_addresses() {
        let mut frame=VoxelFrame::new(VoxelFrameId::new(),WorldScopeId::new(WorldInstanceId::new("world"),"test:terrain"),
            VoxelFrameTransform::new([2.0,3.0,-4.0],[0.1,0.2,0.3,0.9]).unwrap());
        frame.occupied_chunks.insert(ChunkPos::new(-2,3,-4));
        let bytes=encode(&[frame.clone()]).unwrap();
        let decoded=decode(&bytes).unwrap();
        assert_eq!(decoded[0].id,frame.id);
        assert_eq!(decoded[0].occupied_chunks,frame.occupied_chunks);
        assert_eq!(decoded[0].transform.translation(),frame.transform.translation());
        assert!(decoded[0].transform.rotation().dot(frame.transform.rotation()).abs()>1.0-1e-12);
        assert!(decode(&bytes[..bytes.len()-1]).is_err());
    }
}
