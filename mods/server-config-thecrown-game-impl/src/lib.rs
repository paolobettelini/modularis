use server_config_api::ServerConfigApi;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::task::JoinHandle;

pub struct TheCrownGameServerConfigImpl;

impl TheCrownGameServerConfigImpl {
    pub fn init() -> Self { Self }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl ServerConfigApi for TheCrownGameServerConfigImpl {
    fn bind_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 10000)
    }
}

