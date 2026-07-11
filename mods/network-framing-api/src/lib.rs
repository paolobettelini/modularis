use std::{
    collections::VecDeque,
    io::{self, Read, Write},
};

pub const MAX_FRAME_BYTES: usize = 1_048_576;
const HEADER_BYTES: usize = 4;

pub fn write_frame(writer: &mut impl Write, payload: &[u8]) -> io::Result<()> {
    let frame = encode_frame(payload)?;
    writer.write_all(&frame)?;
    writer.flush()
}

pub fn encode_frame(payload: &[u8]) -> io::Result<Vec<u8>> {
    if payload.len() > MAX_FRAME_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("network frame too large: {} bytes", payload.len()),
        ));
    }
    let mut frame = Vec::with_capacity(HEADER_BYTES + payload.len());
    frame.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    frame.extend_from_slice(payload);
    Ok(frame)
}

pub fn flush_queued_frames(
    writer: &mut impl Write,
    queued: &mut VecDeque<Vec<u8>>,
    current: &mut Vec<u8>,
    offset: &mut usize,
) -> io::Result<bool> {
    loop {
        if current.is_empty() {
            let Some(next) = queued.pop_front() else {
                return Ok(true);
            };
            *current = next;
            *offset = 0;
        }

        while *offset < current.len() {
            match writer.write(&current[*offset..]) {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "TCP stream wrote zero bytes",
                    ));
                }
                Ok(written) => *offset += written,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => return Ok(false),
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(error),
            }
        }

        current.clear();
        *offset = 0;
    }
}

pub fn read_available(reader: &mut impl Read, buffer: &mut Vec<u8>) -> io::Result<bool> {
    let mut chunk = [0_u8; 16 * 1024];
    loop {
        match reader.read(&mut chunk) {
            Ok(0) => return Ok(false),
            Ok(length) => buffer.extend_from_slice(&chunk[..length]),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => return Ok(true),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        }
    }
}

pub fn drain_frames(buffer: &mut Vec<u8>) -> io::Result<Vec<Vec<u8>>> {
    let mut frames = Vec::new();
    let mut cursor = 0;
    while buffer.len().saturating_sub(cursor) >= HEADER_BYTES {
        let length = u32::from_be_bytes(
            buffer[cursor..cursor + HEADER_BYTES]
                .try_into()
                .expect("header slice length is fixed"),
        ) as usize;
        if length > MAX_FRAME_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("network frame too large: {length} bytes"),
            ));
        }
        let frame_start = cursor + HEADER_BYTES;
        let frame_end = frame_start + length;
        if buffer.len() < frame_end {
            break;
        }
        frames.push(buffer[frame_start..frame_end].to_vec());
        cursor = frame_end;
    }
    if cursor > 0 {
        buffer.drain(..cursor);
    }
    Ok(frames)
}
