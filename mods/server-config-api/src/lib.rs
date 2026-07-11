use std::net::SocketAddr;

pub trait ServerConfigApi: Send + Sync + 'static {
    fn bind_address() -> SocketAddr;
}
