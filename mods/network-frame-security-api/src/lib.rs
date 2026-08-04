use bevy::prelude::*;
use std::{
    collections::HashMap,
    io,
    net::SocketAddr,
    sync::{Arc, Mutex, RwLock},
};

pub trait SecureFrameCodec: Send + Sync + 'static {
    fn encode(&mut self, plaintext: &[u8]) -> io::Result<Vec<u8>>;
    fn decode_candidate(&self, frame: &[u8]) -> io::Result<Vec<u8>>;
    fn commit_inbound(&mut self) -> io::Result<()>;
}

#[derive(Default)]
enum FrameMode {
    #[default]
    Plaintext,
    Paused,
    Secure(Box<dyn SecureFrameCodec>),
    Failed,
}

impl FrameMode {
    fn encode(&mut self, plaintext: &[u8]) -> io::Result<Vec<u8>> {
        match self {
            Self::Plaintext => Ok(plaintext.to_vec()),
            Self::Secure(codec) => codec.encode(plaintext),
            Self::Paused => Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "secure channel activation is pending",
            )),
            Self::Failed => Err(io::Error::new(
                io::ErrorKind::ConnectionAborted,
                "secure channel is closed",
            )),
        }
    }

    fn decode_candidate(&self, frame: &[u8]) -> io::Result<Option<Vec<u8>>> {
        match self {
            Self::Plaintext => Ok(Some(frame.to_vec())),
            Self::Paused => Ok(None),
            Self::Secure(codec) => codec.decode_candidate(frame).map(Some),
            Self::Failed => Err(io::Error::new(
                io::ErrorKind::ConnectionAborted,
                "secure channel is closed",
            )),
        }
    }

    fn commit_inbound(&mut self) -> io::Result<()> {
        if let Self::Secure(codec) = self {
            codec.commit_inbound()?;
        }
        Ok(())
    }

    fn is_plaintext(&self) -> bool {
        matches!(self, Self::Plaintext)
    }

    fn can_decode(&self) -> bool {
        !matches!(self, Self::Paused | Self::Failed)
    }
}

#[derive(Resource, Clone, Default)]
pub struct ClientFrameSecurity(Arc<Mutex<FrameMode>>);

impl ClientFrameSecurity {
    pub fn reset_plaintext(&self) {
        *self.0.lock().expect("client frame security lock poisoned") = FrameMode::Plaintext;
    }

    pub fn pause(&self) {
        *self.0.lock().expect("client frame security lock poisoned") = FrameMode::Paused;
    }

    pub fn activate(&self, codec: impl SecureFrameCodec) {
        *self.0.lock().expect("client frame security lock poisoned") =
            FrameMode::Secure(Box::new(codec));
    }

    pub fn fail(&self) {
        *self.0.lock().expect("client frame security lock poisoned") = FrameMode::Failed;
    }

    pub fn encode(&self, plaintext: &[u8]) -> io::Result<Vec<u8>> {
        self.0
            .lock()
            .expect("client frame security lock poisoned")
            .encode(plaintext)
    }

    pub fn can_decode(&self) -> bool {
        self.0
            .lock()
            .expect("client frame security lock poisoned")
            .can_decode()
    }

    pub fn is_plaintext(&self) -> bool {
        self.0
            .lock()
            .expect("client frame security lock poisoned")
            .is_plaintext()
    }

    pub fn decode_candidate(&self, frame: &[u8]) -> io::Result<Option<Vec<u8>>> {
        self.0
            .lock()
            .expect("client frame security lock poisoned")
            .decode_candidate(frame)
    }

    pub fn commit_inbound(&self) -> io::Result<()> {
        self.0
            .lock()
            .expect("client frame security lock poisoned")
            .commit_inbound()
    }
}

type SharedFrameMode = Arc<Mutex<FrameMode>>;

#[derive(Resource, Clone, Default)]
pub struct ServerFrameSecurity(Arc<RwLock<HashMap<SocketAddr, SharedFrameMode>>>);

impl ServerFrameSecurity {
    pub fn register_plaintext(&self, address: SocketAddr) {
        self.0
            .write()
            .expect("server frame security map poisoned")
            .insert(address, Arc::new(Mutex::new(FrameMode::Plaintext)));
    }

    pub fn remove(&self, address: SocketAddr) {
        self.0
            .write()
            .expect("server frame security map poisoned")
            .remove(&address);
    }

    pub fn pause(&self, address: SocketAddr) -> io::Result<()> {
        self.with_mode(address, |mode| {
            *mode = FrameMode::Paused;
            Ok(())
        })
    }

    pub fn activate(&self, address: SocketAddr, codec: impl SecureFrameCodec) -> io::Result<()> {
        self.with_mode(address, |mode| {
            *mode = FrameMode::Secure(Box::new(codec));
            Ok(())
        })
    }

    pub fn fail(&self, address: SocketAddr) {
        let _ = self.with_mode(address, |mode| {
            *mode = FrameMode::Failed;
            Ok(())
        });
    }

    pub fn encode(&self, address: SocketAddr, plaintext: &[u8]) -> io::Result<Vec<u8>> {
        self.with_mode(address, |mode| mode.encode(plaintext))
    }

    pub fn can_decode(&self, address: SocketAddr) -> bool {
        self.with_mode(address, |mode| Ok(mode.can_decode()))
            .unwrap_or(false)
    }

    pub fn is_plaintext(&self, address: SocketAddr) -> bool {
        self.with_mode(address, |mode| Ok(mode.is_plaintext()))
            .unwrap_or(false)
    }

    pub fn decode_candidate(
        &self,
        address: SocketAddr,
        frame: &[u8],
    ) -> io::Result<Option<Vec<u8>>> {
        self.with_mode(address, |mode| mode.decode_candidate(frame))
    }

    pub fn commit_inbound(&self, address: SocketAddr) -> io::Result<()> {
        self.with_mode(address, FrameMode::commit_inbound)
    }

    fn with_mode<T>(
        &self,
        address: SocketAddr,
        operation: impl FnOnce(&mut FrameMode) -> io::Result<T>,
    ) -> io::Result<T> {
        let mode = self
            .0
            .read()
            .expect("server frame security map poisoned")
            .get(&address)
            .cloned()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotConnected,
                    format!("no frame security state for {address}"),
                )
            })?;
        let mut mode = mode.lock().expect("server frame security lock poisoned");
        operation(&mut mode)
    }
}
