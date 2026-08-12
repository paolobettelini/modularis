use block_component_api::{BlockComponentError, EncodedBlockComponent};

const MAGIC: &[u8; 4] = b"PWBC";
const FORMAT_VERSION: u16 = 1;

pub fn encode(records: &[EncodedBlockComponent]) -> Result<Vec<u8>, BlockComponentError> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    bytes.extend_from_slice(&u32::try_from(records.len()).map_err(|_| BlockComponentError("too many block component records".into()))?.to_le_bytes());
    for record in records {
        let id = record.component_id.as_bytes();
        bytes.extend_from_slice(&record.local_index.to_le_bytes());
        bytes.extend_from_slice(&u16::try_from(id.len()).map_err(|_| BlockComponentError("component ID is too long".into()))?.to_le_bytes());
        bytes.extend_from_slice(id);
        bytes.extend_from_slice(&record.version.to_le_bytes());
        bytes.extend_from_slice(&u32::try_from(record.payload.len()).map_err(|_| BlockComponentError("component payload is too large".into()))?.to_le_bytes());
        bytes.extend_from_slice(&record.payload);
    }
    Ok(bytes)
}

pub fn decode(bytes: &[u8]) -> Result<Vec<EncodedBlockComponent>, BlockComponentError> {
    let mut cursor = Cursor { bytes, offset: 0 };
    if cursor.take(4)? != MAGIC { return Err(BlockComponentError("invalid block component magic".into())); }
    if cursor.u16()? != FORMAT_VERSION { return Err(BlockComponentError("unsupported block component format version".into())); }
    let count = cursor.u32()? as usize;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        let local_index = cursor.u16()?;
        let id_len = cursor.u16()? as usize;
        let component_id = std::str::from_utf8(cursor.take(id_len)?).map_err(|error| BlockComponentError(error.to_string()))?.to_string();
        let version = cursor.u32()?;
        let payload_len = cursor.u32()? as usize;
        let payload = cursor.take(payload_len)?.to_vec();
        records.push(EncodedBlockComponent { local_index, component_id, version, payload });
    }
    if cursor.offset != bytes.len() { return Err(BlockComponentError("trailing block component bytes".into())); }
    Ok(records)
}

struct Cursor<'a> { bytes: &'a [u8], offset: usize }

impl<'a> Cursor<'a> {
    fn take(&mut self, count: usize) -> Result<&'a [u8], BlockComponentError> {
        let end = self.offset.checked_add(count).ok_or_else(|| BlockComponentError("component payload overflow".into()))?;
        let value = self.bytes.get(self.offset..end).ok_or_else(|| BlockComponentError("truncated component payload".into()))?;
        self.offset = end;
        Ok(value)
    }
    fn u16(&mut self) -> Result<u16, BlockComponentError> { Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap())) }
    fn u32(&mut self) -> Result<u32, BlockComponentError> { Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap())) }
}

