pub trait ClientPlayerPhysicsTickApi: Send + Sync + 'static {
    fn ticks_per_second() -> f64;
}
