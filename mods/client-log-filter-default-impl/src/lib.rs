use client_log_filter_api::ClientLogFilterApi;
use tokio::task::JoinHandle;

pub struct ClientLogFilterDefaultImpl;

impl ClientLogFilterDefaultImpl {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientLogFilterApi for ClientLogFilterDefaultImpl {
    fn filter() -> &'static str {
        "wgpu=error,naga=warn,calloop::loop_logic=error"
    }
}
