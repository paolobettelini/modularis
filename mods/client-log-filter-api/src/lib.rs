pub trait ClientLogFilterApi: Send + Sync + 'static {
    fn filter() -> &'static str;
}
