use server_config_api::ServerConfigApi;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::task::JoinHandle;

pub struct TheCrownAuthServerConfigImpl;

impl TheCrownAuthServerConfigImpl {
    pub fn init() -> Self { Self }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl ServerConfigApi for TheCrownAuthServerConfigImpl {
    fn bind_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 9999)
    }
}

