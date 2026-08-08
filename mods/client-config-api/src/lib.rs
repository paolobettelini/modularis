pub trait ClientConfigApi: Send + Sync + 'static {
    fn window_title() -> &'static str;
    fn default_server_address() -> &'static str;
}
